use std::path::{Path, PathBuf};

use clap::Parser;

/// Run neovim inside docker container
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RawArgs {
    /// File path to be opened.
    path: Option<String>,
    #[arg(short, long)]
    update: bool,
}

#[derive(Clone)]
pub struct Args<T: AsRef<Path>> {
    pub path: Option<T>,
    pub update: bool,
}

impl From<RawArgs> for Args<PathBuf> {
    fn from(value: RawArgs) -> Self {
        Args {
            path: value.path.map(|p| Path::new(p.as_str()).to_path_buf()),
            update: value.update,
        }
    }
}
