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

use std::{collections::HashMap, path::Path};

use action::{pull_image, run_container, update_binary};
use anyhow::Result;
use clap::Parser;
use constants::UID;
use container_config::{image_name, ContainerImageConfig};
use directory_state::{DirectoryStateProvider, DirectoryStateProviderImpl};
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
    let compose_yaml = DirectoryStateProviderImpl.file_content(&Path::new("compose.yml"))?;
    let deserialized: Compose = serde_yaml::from_str(&compose_yaml)?;
    println!("{:?}", deserialized);
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
