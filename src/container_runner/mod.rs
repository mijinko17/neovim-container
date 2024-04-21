mod command_executor_cor;

use std::path::PathBuf;

use anyhow::{bail, Result};

use crate::directory_state::DirectoryStateProvider;

use self::command_executor_cor::{CreateNvimCommandExecutorCor, DirectoryCor, FileCor};

pub struct RunContainerArg {
    pub image: String,
    pub volume: Vec<(PathBuf, PathBuf)>,
    pub host_path: Option<PathBuf>,
}

pub fn run_container(
    args: RunContainerArg,
    dir_state_provider: impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    let cors: Vec<Box<dyn CreateNvimCommandExecutorCor>> = vec![
        Box::new(DirectoryCor {
            dir_state_provider: &dir_state_provider,
            container_name,
        }),
        Box::new(FileCor {
            dir_state_provider: &dir_state_provider,
            container_name,
        }),
    ];
    if let Some(executor) = cors.into_iter().find(|cor| cor.is_responsible(&args)) {
        executor.create(args)?.execute()
    } else {
        bail!("No method was found.")
    }
}
