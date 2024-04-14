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
use clipboard::{clean_named_pipe, setup_clipboard};
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
        let result = setup_clipboard(&DirectoryStateProviderImpl, &container_name)
            .and_then(|_| run_container(args, DirectoryStateProviderImpl, &container_name));
        let _ = clean_named_pipe(&DirectoryStateProviderImpl, &container_name);
        result
    }
}

fn random_contaniner_name() -> String {
    "hoge".to_string()
}
