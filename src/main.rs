mod action;
mod cli;
mod clipboard;
mod command_executor;
mod constants;
mod container_config;
mod container_runner;
mod directory_state;
mod path;
mod telekasten;
mod terminal_command;
mod update_binary;

use action::{pull_image, run_container, update_binary};
use anyhow::Result;
use clap::Parser;
use constants::UID;
use container_config::{image_name, ContainerImageConfig};
use rand::random;

use crate::cli::{Args, RawArgs};

fn main() -> Result<()> {
    let args = Args::from(RawArgs::parse());
    if args.update {
        update_binary()
    } else if args.pull {
        pull_image(image_name(ContainerImageConfig { uid: UID }))
    } else {
        run_container(args)
    }
}

fn random_contaniner_name() -> String {
    let (w, x, y, z): (u8, u8, u8, u8) = (
        random::<u8>(),
        random::<u8>(),
        random::<u8>(),
        random::<u8>(),
    );
    format!("neovim-{w:0>2x}{x:0>2x}{y:0>2x}{z:0>2x}")
}
