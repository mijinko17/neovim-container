pub static NEOVIM_IMAGE_PREFIX: &str = "mijinko17/neovim-container";

#[cfg(feature = "develop")]
pub static NEOVIM_IMAGE_TAG: &str = "develop";
#[cfg(not(feature = "develop"))]
pub static NEOVIM_IMAGE_TAG: &str = "latest";

pub static UID: u16 = 1000;
