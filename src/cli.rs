use std::path::{Path, PathBuf};

use clap::Parser;

/// Run neovim inside docker container
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RawArgs {
    #[arg(short, long, default_value_t = false, hide = true)]
    develop: bool,
    /// File path to be opened.
    path: Option<String>,
}

#[derive(Clone)]
pub struct Args<T: AsRef<Path>> {
    pub develop: bool,
    pub path: Option<T>,
}

impl From<RawArgs> for Args<PathBuf> {
    fn from(value: RawArgs) -> Self {
        Args {
            develop: value.develop,
            path: value.path.map(|p| Path::new(p.as_str()).to_path_buf()),
        }
    }
}