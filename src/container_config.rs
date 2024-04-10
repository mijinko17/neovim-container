use crate::constants::{NEOVIM_IMAGE_PREFIX, NEOVIM_IMAGE_TAG};
pub struct ContainerImageConfig {
    pub uid: u32,
}

pub fn image_name(config: ContainerImageConfig) -> String {
    format!(
        "{}-uid-{}:{}",
        NEOVIM_IMAGE_PREFIX, config.uid, NEOVIM_IMAGE_TAG
    )
}
