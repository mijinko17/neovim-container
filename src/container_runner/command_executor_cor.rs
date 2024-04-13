use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::{
    cli::Args,
    command_executor::{NvimCommandExecutor, VolumeArg},
    container_config::{image_name, ContainerImageConfig},
    directory_state::DirectoryStateProvider,
};

pub trait CreateNvimCommandExecutorCor {
    fn create(&self, args: Args<PathBuf>) -> Result<NvimCommandExecutor<PathBuf, PathBuf>>;
}

pub struct DirectoryCor<'a, T: DirectoryStateProvider> {
    pub dir_state_provider: &'a T,
}

impl<'a, T> CreateNvimCommandExecutorCor for DirectoryCor<'a, T>
where
    T: DirectoryStateProvider,
{
    fn create(&self, args: Args<PathBuf>) -> Result<NvimCommandExecutor<PathBuf, PathBuf>> {
        if args.path.is_some() {
            bail!("Target path is specified, so not resonsible.");
        }
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
            ],
            work_dir,
            target_file_path: None as Option<PathBuf>,
        })
    }
}

pub struct FileCor<'a, T: DirectoryStateProvider> {
    pub dir_state_provider: &'a T,
}

impl<'a, T> CreateNvimCommandExecutorCor for FileCor<'a, T>
where
    T: DirectoryStateProvider,
{
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
            ],
            work_dir,
            target_file_path,
        })
    }
}
