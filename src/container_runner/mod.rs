mod command_executor_cor;

use std::path::PathBuf;

use crate::{cli::Args, directory_state::DirectoryStateProvider};

use self::command_executor_cor::{CreateNvimCommandExecutorCor, DirectoryCor, FileCor};

pub fn run_container(args: Args<PathBuf>, dir_state_provider: impl DirectoryStateProvider) {
    let cors: Vec<Box<dyn CreateNvimCommandExecutorCor>> = vec![
        Box::new(DirectoryCor {
            dir_state_provider: &dir_state_provider,
        }),
        Box::new(FileCor {
            dir_state_provider: &dir_state_provider,
        }),
    ];
    if let Some(executor) = cors.into_iter().find_map(|cor| cor.create(args.clone())) {
        executor.execute()
    }
}
