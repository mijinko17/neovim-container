mod create_command_cor;

use std::path::PathBuf;

use anyhow::{bail, Result};

use crate::interface::directory_state::DirectoryStateProvider;

use self::create_command_cor::{CreateNvimCommandCor, DirectoryCor, FileCor};

pub struct RunNvimContainerArg {
    pub image: String,
    pub volume: Vec<(PathBuf, PathBuf)>,
    pub host_path: Option<PathBuf>,
    pub container_name: String,
}

pub fn run_container(
    args: RunNvimContainerArg,
    dir_state_provider: impl DirectoryStateProvider,
) -> Result<()> {
    let cors: Vec<Box<dyn CreateNvimCommandCor>> = vec![
        Box::new(DirectoryCor {
            dir_state_provider: &dir_state_provider,
        }),
        Box::new(FileCor {
            dir_state_provider: &dir_state_provider,
        }),
    ];
    if let Some(executor) = cors.into_iter().find(|cor| cor.is_responsible(&args)) {
        executor.create(args)?.execute()
    } else {
        bail!("No method was found.")
    }
}
