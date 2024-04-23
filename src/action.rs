use std::path::PathBuf;

use anyhow::Result;

use crate::clipboard::clean_named_pipe;

use crate::config_reader::ConfigReader;
use crate::container_runner::RunContainerArg;
use crate::{
    cli::Args, clipboard::setup_clipboard, directory_state::DirectoryStateProviderImpl,
    random_contaniner_name, telekasten::setup_for_telekasten,
    terminal_command::pull_image_command::PullImageComand,
};

pub fn pull_image(image: String) -> Result<()> {
    PullImageComand::new(image).execute()
}

pub fn update_binary() -> Result<()> {
    crate::update_binary::update_binary()
}

pub fn run_container(args: Args<PathBuf>, config_reader: impl ConfigReader) -> Result<()> {
    let _ = setup_for_telekasten(&DirectoryStateProviderImpl);
    let container_name = random_contaniner_name();
    let config = config_reader.config("default")?;
    let run_container_arg = RunContainerArg {
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