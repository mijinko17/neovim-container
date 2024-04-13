mod cli;
mod clipboard;
mod command_executor;
mod constants;
mod container_config;
mod container_runner;
mod directory_state;
mod path;
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
        setup_clipboard(&DirectoryStateProviderImpl)?;
        run_container(args, DirectoryStateProviderImpl)
    }
}
