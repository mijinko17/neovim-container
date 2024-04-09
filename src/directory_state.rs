use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use dirs::home_dir;

pub trait DirectoryStateProvider {
    fn current_dir(&self) -> Option<PathBuf>;
    fn home_dir(&self) -> Option<PathBuf>;
    fn absolute_path(&self, relative_path: &impl AsRef<Path>) -> PathBuf;
}

pub struct DirectoryStateProviderImpl;

impl DirectoryStateProvider for DirectoryStateProviderImpl {
    fn current_dir(&self) -> Option<PathBuf> {
        current_dir().ok()
    }

    fn home_dir(&self) -> Option<PathBuf> {
        home_dir()
    }

    fn absolute_path(&self, relative_path: &impl AsRef<std::path::Path>) -> PathBuf {
        std::fs::canonicalize(relative_path).unwrap()
    }
}
