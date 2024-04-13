use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use dirs::home_dir;

pub trait DirectoryStateProvider {
    fn current_dir(&self) -> Result<PathBuf>;
    fn home_dir(&self) -> Result<PathBuf>;
    fn absolute_path(&self, relative_path: &impl AsRef<Path>) -> Result<PathBuf>;
}

pub struct DirectoryStateProviderImpl;

impl DirectoryStateProvider for DirectoryStateProviderImpl {
    fn current_dir(&self) -> Result<PathBuf> {
        Ok(current_dir()?)
    }

    fn home_dir(&self) -> Result<PathBuf> {
        home_dir().context("Failed to get home directory.")
    }

    fn absolute_path(&self, relative_path: &impl AsRef<std::path::Path>) -> Result<PathBuf> {
        Ok(std::fs::canonicalize(relative_path)?)
    }
}
