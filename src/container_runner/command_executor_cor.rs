use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::{
    clipboard::{
        from_container::clipboard_named_pipe_from_container_path,
        from_host::clipboard_named_pipe_from_host_path,
    },
    command_executor::{NvimCommandExecutor, VolumeArg},
    directory_state::DirectoryStateProvider,
};

use super::RunContainerArg;

pub trait CreateNvimCommandExecutorCor {
    fn is_responsible(&self, args: &RunContainerArg) -> bool;
    fn create(&self, args: RunContainerArg) -> Result<NvimCommandExecutor<PathBuf, PathBuf>>;
}

pub struct DirectoryCor<'a, 'b, T: DirectoryStateProvider> {
    pub dir_state_provider: &'a T,
    pub container_name: &'b str,
}

impl<'a, 'b, T> CreateNvimCommandExecutorCor for DirectoryCor<'a, 'b, T>
where
    T: DirectoryStateProvider,
{
    fn is_responsible(&self, args: &RunContainerArg) -> bool {
        args.host_path.is_none()
    }
    fn create(&self, args: RunContainerArg) -> Result<NvimCommandExecutor<PathBuf, PathBuf>> {
        let current_dir = self.dir_state_provider.current_dir()?;
        let work_dir = Path::new("/home/host").to_path_buf();
        Ok(NvimCommandExecutor {
            image: args.image,
            container_name: self.container_name.to_string(),
            volumes: [
                args.volume
                    .into_iter()
                    .map(|(host_path, container_path)| {
                        Ok(VolumeArg::new(
                            self.dir_state_provider.expand_home_dir(&host_path)?,
                            self.dir_state_provider.expand_home_dir(&container_path)?,
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?,
                vec![
                    VolumeArg::new(current_dir, Path::new("/home/host")),
                    VolumeArg::new(
                        clipboard_named_pipe_from_host_path(
                            self.dir_state_provider,
                            self.container_name,
                        )?,
                        Path::new("/home/neovim/pipes/clipboard"),
                    ),
                    VolumeArg::new(
                        clipboard_named_pipe_from_container_path(
                            self.dir_state_provider,
                            self.container_name,
                        )?,
                        Path::new("/home/neovim/pipes/from_container"),
                    ),
                ],
            ]
            .concat(),
            work_dir,
            target_file_path: None as Option<PathBuf>,
        })
    }
}

pub struct FileCor<'a, 'b, T: DirectoryStateProvider> {
    pub dir_state_provider: &'a T,
    pub container_name: &'b str,
}

impl<'a, 'b, T> CreateNvimCommandExecutorCor for FileCor<'a, 'b, T>
where
    T: DirectoryStateProvider,
{
    fn is_responsible(&self, args: &RunContainerArg) -> bool {
        args.host_path.is_some()
    }
    fn create(&self, args: RunContainerArg) -> Result<NvimCommandExecutor<PathBuf, PathBuf>> {
        let target = self.dir_state_provider.absolute_path(
            &args
                .host_path
                .context("Failed to convert path to absolute.")?,
        )?;
        let parent_dir = target.parent().context("Failed to get parent directory.")?;
        let target_file_path = Some(
            Path::new("/home/host").join(target.file_name().context("Failed to get file name.")?),
        );
        let work_dir = Path::new("/home/host").to_path_buf();
        Ok(NvimCommandExecutor {
            image: args.image,
            container_name: self.container_name.to_string(),
            volumes: [
                args.volume
                    .into_iter()
                    .map(|(host_path, container_path)| VolumeArg::new(host_path, container_path))
                    .collect(),
                vec![
                    VolumeArg::new(parent_dir, Path::new("/home/host")),
                    VolumeArg::new(
                        clipboard_named_pipe_from_host_path(
                            self.dir_state_provider,
                            self.container_name,
                        )?,
                        Path::new("/home/neovim/pipes/clipboard"),
                    ),
                    VolumeArg::new(
                        clipboard_named_pipe_from_container_path(
                            self.dir_state_provider,
                            self.container_name,
                        )?,
                        Path::new("/home/neovim/pipes/from_container"),
                    ),
                ],
            ]
            .concat(),
            work_dir,
            target_file_path,
        })
    }
}
