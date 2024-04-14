use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::{
    cli::Args,
    clipboard::{
        from_container::clipboard_named_pipe_from_container_path,
        from_host::clipboard_named_pipe_from_host_path,
    },
    command_executor::{NvimCommandExecutor, VolumeArg},
    container_config::{image_name, ContainerImageConfig},
    directory_state::DirectoryStateProvider,
};

pub trait CreateNvimCommandExecutorCor {
    fn is_responsible(&self, args: &Args<PathBuf>) -> bool;
    fn create(&self, args: Args<PathBuf>) -> Result<NvimCommandExecutor<PathBuf, PathBuf>>;
}

pub struct DirectoryCor<'a, 'b, T: DirectoryStateProvider> {
    pub dir_state_provider: &'a T,
    pub container_name: &'b str,
}

impl<'a, 'b, T> CreateNvimCommandExecutorCor for DirectoryCor<'a, 'b, T>
where
    T: DirectoryStateProvider,
{
    fn is_responsible(&self, args: &Args<PathBuf>) -> bool {
        args.path.is_none()
    }
    fn create(&self, _args: Args<PathBuf>) -> Result<NvimCommandExecutor<PathBuf, PathBuf>> {
        let home_dir = self.dir_state_provider.home_dir()?;
        let current_dir = self.dir_state_provider.current_dir()?;
        let work_dir = Path::new("/home/host").to_path_buf();
        Ok(NvimCommandExecutor {
            image: image_name(ContainerImageConfig { uid: 1000 }),
            volumes: vec![
                VolumeArg::new(current_dir, Path::new("/home/host")),
                VolumeArg::new(
                    home_dir.clone().join(Path::new(".gitconfig")),
                    Path::new("/home/neovim/.gitconfig"),
                ),
                VolumeArg::new(
                    home_dir.join(Path::new(".ssh")),
                    Path::new("/home/neovim/.ssh"),
                ),
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
    fn is_responsible(&self, args: &Args<PathBuf>) -> bool {
        args.path.is_some()
    }
    fn create(&self, args: Args<PathBuf>) -> Result<NvimCommandExecutor<PathBuf, PathBuf>> {
        let target = self
            .dir_state_provider
            .absolute_path(&args.path.context("Failed to convert path to absolute.")?)?;
        let parent_dir = target.parent().context("Failed to get parent directory.")?;
        let target_file_path = Some(
            Path::new("/home/host").join(target.file_name().context("Failed to get file name.")?),
        );
        let home_dir = self.dir_state_provider.home_dir()?;
        let work_dir = Path::new("/home/host").to_path_buf();
        Ok(NvimCommandExecutor {
            image: image_name(ContainerImageConfig { uid: 1000 }),
            volumes: vec![
                VolumeArg::new(parent_dir, Path::new("/home/host")),
                VolumeArg::new(
                    home_dir.clone().join(Path::new(".gitconfig")),
                    Path::new("/home/neovim/.gitconfig"),
                ),
                VolumeArg::new(
                    home_dir.join(Path::new(".ssh")),
                    Path::new("/home/neovim/.ssh"),
                ),
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
            work_dir,
            target_file_path,
        })
    }
}
