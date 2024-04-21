use std::{
    env::{self, current_dir},
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
    fn remove_file(&self, path: &impl AsRef<Path>) -> Result<()>;
    fn config_dir(&self) -> Result<PathBuf>;
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

    fn remove_file(&self, path: &impl AsRef<Path>) -> Result<()> {
        Ok(std::fs::remove_file(path)?)
    }

    fn config_dir(&self) -> Result<PathBuf> {
        env::var_os("XDG_CONFIG_HOME")
            .map(|a| Path::new(&a).to_path_buf())
            .or_else(|| home_dir().map(|h| h.join(".config")))
            .context("Unable to get config directory.")
    }
}

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
#[derive(Default)]
pub struct DirectoryStateProviderMock {
    config_dir: Option<PathBuf>,
    file_content_map: HashMap<PathBuf, String>,
}

#[cfg(test)]
impl DirectoryStateProviderMock {
    pub fn with_config_dir(self, path: impl AsRef<Path>) -> Self {
        Self {
            config_dir: Some(path.as_ref().to_path_buf()),
            file_content_map: self.file_content_map,
        }
    }

    pub fn with_file_content(mut self, path: impl AsRef<Path>, content: String) -> Self {
        self.file_content_map
            .insert(path.as_ref().to_path_buf(), content);
        self
    }
}

#[cfg(test)]
impl DirectoryStateProvider for DirectoryStateProviderMock {
    fn current_dir(&self) -> Result<PathBuf> {
        todo!()
    }

    fn home_dir(&self) -> Result<PathBuf> {
        todo!()
    }

    fn absolute_path(&self, _relative_path: &impl AsRef<Path>) -> Result<PathBuf> {
        todo!()
    }

    fn create_named_pipes(&self, _path: &impl AsRef<Path>) -> Result<()> {
        todo!()
    }

    fn create_directory(&self, _path: &impl AsRef<Path>) -> Result<()> {
        todo!()
    }

    fn file_content(&self, path: &impl AsRef<Path>) -> Result<String> {
        self.file_content_map
            .get(&path.as_ref().to_path_buf())
            .map(|str| str.to_string())
            .context("Unexpected path is specified.")
    }

    fn write_file(&self, _path: &impl AsRef<Path>, _content: &[u8]) -> Result<()> {
        todo!()
    }

    fn remove_file(&self, _path: &impl AsRef<Path>) -> Result<()> {
        todo!()
    }

    fn config_dir(&self) -> Result<PathBuf> {
        self.config_dir
            .clone()
            .context("Config Directory is not defined.")
    }
}
