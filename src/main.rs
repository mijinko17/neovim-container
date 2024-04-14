mod cli;
mod clipboard;
mod command_executor;
mod constants;
mod container_config;
mod container_runner;
mod directory_state;
mod path;
mod terminal_command;
mod update_binary;

use anyhow::Result;
use clap::Parser;
use clipboard::setup_clipboard;
use directory_state::DirectoryStateProviderImpl;
use update_binary::update_binary;

use crate::cli::{Args, RawArgs};
use crate::container_runner::run_container;

fn main() -> Result<()> {
    let args = Args::from(RawArgs::parse());
    if args.update {
        update_binary()
    } else {
        let container_name = random_contaniner_name();
        setup_clipboard(&DirectoryStateProviderImpl, &container_name)?;
        run_container(args, DirectoryStateProviderImpl, &container_name)
    }
}

fn random_contaniner_name() -> String {
    "hoge".to_string()
}
