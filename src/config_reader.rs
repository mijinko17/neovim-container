use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::directory_state::DirectoryStateProvider;

pub trait ConfigReader {
    fn config(&self, service_name: &str) -> anyhow::Result<ContainerService>;
}

pub struct ConfigReaderImpl<T: DirectoryStateProvider> {
    dir_state: T,
}

impl<T: DirectoryStateProvider> ConfigReaderImpl<T> {
    pub fn new(dir_state: T) -> Self {
        Self { dir_state }
    }
}

impl<T: DirectoryStateProvider> ConfigReader for ConfigReaderImpl<T> {
    fn config(&self, service_name: &str) -> anyhow::Result<ContainerService> {
        let compose_yaml = self.dir_state.file_content(
            &self
                .dir_state
                .config_dir()?
                .join("neovim-container/neovim-container.yml"),
        )?;
        serde_yaml::from_str::<RawNeovimContainerConfig>(&compose_yaml)?
            .services
            .remove(service_name)
            .unwrap()
            .try_into()
    }
}

pub struct ContainerService {
    pub image: String,
    pub volumes: Vec<(PathBuf, PathBuf)>,
}

impl TryFrom<RawService> for ContainerService {
    type Error = anyhow::Error;
    fn try_from(value: RawService) -> Result<Self, Self::Error> {
        Ok(ContainerService {
            image: value.image,
            volumes: value
                .volumes
                .map(|vec_string| {
                    vec_string
                        .into_iter()
                        .map(split)
                        .collect::<anyhow::Result<Vec<_>>>()
                })
                .unwrap_or(Ok(vec![]))?,
        })
    }
}

fn split(volume: String) -> anyhow::Result<(PathBuf, PathBuf)> {
    let a = volume.split(':').collect::<Vec<_>>();
    Ok((
        Path::new(
            a.first()
                .context("Parsing volumes is failed. Host path is not specified.")?,
        )
        .to_path_buf(),
        Path::new(
            a.get(1)
                .context("Parsing volumes is failed. Container path is not specified.")?,
        )
        .to_path_buf(),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
struct RawService {
    image: String,
    volumes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawNeovimContainerConfig {
    services: HashMap<String, RawService>,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::directory_state::DirectoryStateProviderMock;

    use super::{ConfigReader, ConfigReaderImpl};

    #[test]
    fn get_config_from_yaml_successfully() -> anyhow::Result<()> {
        let yaml = "
        services:
          default:
            image: mijinko17/neovim-container
            volumes:
                - ~/zettelkasten:/home/neovim/zettelkasten
                - ~/.ssh:/home/neovim/.ssh
                - ~/.gitconfig:/home/neovim/.gitconfig
        ";
        let dir_state = DirectoryStateProviderMock::default()
            .with_config_dir(Path::new("/home/neovim/.config"))
            .with_file_content(
                Path::new("/home/neovim/.config/neovim-container/neovim-container.yml"),
                yaml,
            );
        let config_reader = ConfigReaderImpl { dir_state };
        let config = config_reader.config("default")?;
        assert_eq!(config.image, "mijinko17/neovim-container");
        assert_eq!(
            config.volumes,
            vec![
                (
                    Path::new("~/zettelkasten").to_path_buf(),
                    Path::new("/home/neovim/zettelkasten").to_path_buf()
                ),
                (
                    Path::new("~/.ssh").to_path_buf(),
                    Path::new("/home/neovim/.ssh").to_path_buf()
                ),
                (
                    Path::new("~/.gitconfig").to_path_buf(),
                    Path::new("/home/neovim/.gitconfig").to_path_buf()
                )
            ]
        );
        Ok(())
    }
}
