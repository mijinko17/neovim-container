use std::{
    env::current_dir,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use dirs::home_dir;
use nix::{sys::stat, unistd};

pub trait DirectoryStateProvider: Sync {
    fn current_dir(&self) -> Result<PathBuf>;
    fn home_dir(&self) -> Result<PathBuf>;
    fn absolute_path(&self, relative_path: &impl AsRef<Path>) -> Result<PathBuf>;
    fn create_named_pipes(&self, path: &impl AsRef<Path>) -> Result<()>;
    fn create_directory(&self, path: &impl AsRef<Path>) -> Result<()>;
    fn file_content(&self, path: &impl AsRef<Path>) -> Result<String>;
    fn write_file(&self, path: &impl AsRef<Path>, content: &[u8]) -> Result<()>;
}

pub struct DirectoryStateProviderImpl;

impl DirectoryStateProvider for DirectoryStateProviderImpl {
    fn current_dir(&self) -> Result<PathBuf> {
        Ok(current_dir()?)
    }

    fn home_dir(&self) -> Result<PathBuf> {
        home_dir().context("Failed to get home directory.")
    }

    fn absolute_path(&self, relative_path: &impl AsRef<Path>) -> Result<PathBuf> {
        Ok(std::fs::canonicalize(relative_path)?)
    }

    fn create_named_pipes(&self, path: &impl AsRef<Path>) -> Result<()> {
        Ok(unistd::mkfifo(path.as_ref(), stat::Mode::S_IRWXU)?)
    }

    fn create_directory(&self, path: &impl AsRef<Path>) -> Result<()> {
        Ok(fs::create_dir(path)?)
    }

    fn file_content(&self, path: &impl AsRef<Path>) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }

    fn write_file(&self, path: &impl AsRef<Path>, content: &[u8]) -> Result<()> {
        let mut file = fs::File::create(path)?;
        Ok(file.write_all(content)?)
    }
}
