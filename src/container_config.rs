use crate::constants::NEOVIM_IMAGE_PREFIX;
pub struct ContainerConfig {
    pub uid: u32,
    pub develop: bool,
}

pub fn image_name(config: ContainerConfig) -> String {
    format!(
        "{}-uid-{}:{}",
        NEOVIM_IMAGE_PREFIX,
        config.uid,
        if config.develop { "develop" } else { "latest" }
    )
}
