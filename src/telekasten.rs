use anyhow::Result;

use crate::interface::directory_state::DirectoryStateProvider;

pub fn setup_for_telekasten(dir_state: &impl DirectoryStateProvider) -> Result<()> {
    dir_state.create_directory(&dir_state.home_dir()?.join("zettelkasten"))
}
