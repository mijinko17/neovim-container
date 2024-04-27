mod action;
mod cli;
mod clipboard;
mod constants;
mod container_runner;
mod interface;
mod path;
mod update_binary;

use std::collections::HashMap;

use action::{pull_image, run_container, update_binary};
use anyhow::Result;
use clap::Parser;
use interface::{config_reader::ConfigReaderImpl, directory_state::DirectoryStateProviderImpl};
use rand::random;
use serde::{Deserialize, Serialize};

use crate::cli::{Args, RawArgs};

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    image: String,
    volumes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Compose {
    services: HashMap<String, Service>,
}

fn main() -> Result<()> {
    let args = Args::from(RawArgs::parse());
    if args.update {
        update_binary()
    } else if args.pull {
        pull_image(
            &args.service,
            ConfigReaderImpl::new(DirectoryStateProviderImpl),
        )
    } else {
        run_container(args, ConfigReaderImpl::new(DirectoryStateProviderImpl))
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
