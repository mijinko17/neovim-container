use std::path::PathBuf;

use anyhow::Result;

use crate::clipboard::clean_named_pipe;

use crate::container_runner::RunNvimContainerArg;
use crate::interface::config_reader::ConfigReader;
use crate::interface::directory_state::DirectoryStateProviderImpl;
use crate::interface::terminal_command::pull_image_command::PullImageComand;
use crate::interface::terminal_command::run_from_string_command::RunFromStringCommand;
use crate::{cli::Args, clipboard::setup_clipboard, random_contaniner_name};

pub fn pull_image(image: String) -> Result<()> {
    PullImageComand::new(image).execute()
}

pub fn update_binary() -> Result<()> {
    crate::update_binary::update_binary()
}

pub fn run_container(args: Args<PathBuf>, config_reader: impl ConfigReader) -> Result<()> {
    let container_name = random_contaniner_name();
    let config = config_reader.config(&args.service)?;
    if let Some(command) = config.before_command {
        let _ = RunFromStringCommand::new(command).execute();
    }
    let run_container_arg = RunNvimContainerArg {
        image: config.image,
        volume: config.volumes,
        host_path: args.path,
        container_name: container_name.clone(),
    };
    let result =
        setup_clipboard(&DirectoryStateProviderImpl, container_name.as_str()).and_then(|_| {
            crate::container_runner::run_container(run_container_arg, DirectoryStateProviderImpl)
        });
    let _ = clean_named_pipe(&DirectoryStateProviderImpl, &container_name);
    result
}
