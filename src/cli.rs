use std::path::{Path, PathBuf};

use clap::Parser;

use crate::constants::DEFAULT_SERVICE;

/// Run neovim inside docker container
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RawArgs {
    /// File path to be opened.
    path: Option<String>,
    /// Update binary.
    #[arg(short, long)]
    update: bool,
    /// Pull image.
    #[arg(short, long)]
    pull: bool,
    /// Service you want to start. You can describe service in config file.
    #[arg(short, long, default_value = DEFAULT_SERVICE)]
    service: String,
}

#[derive(Clone)]
pub struct Args<T: AsRef<Path>> {
    pub path: Option<T>,
    pub update: bool,
    pub pull: bool,
    pub service: String,
}

impl From<RawArgs> for Args<PathBuf> {
    fn from(value: RawArgs) -> Self {
        Args {
            path: value.path.map(|p| Path::new(p.as_str()).to_path_buf()),
            update: value.update,
            pull: value.pull,
            service: value.service,
        }
    }
}
