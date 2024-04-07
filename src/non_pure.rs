use std::{env::current_dir, path::PathBuf};

use dirs::home_dir;

use crate::container_runner::DirectoryStateProvider;

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
