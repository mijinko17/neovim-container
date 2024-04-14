pub mod from_container;
pub mod from_host;

use std::path::PathBuf;

use anyhow::Result;

use crate::directory_state::DirectoryStateProvider;

use self::{from_container::setup_clipboard_from_container, from_host::setup_clipboard_from_host};

pub fn setup_clipboard(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    setup_clipboard_from_host(dir_state, container_name)?;
    setup_clipboard_from_container(dir_state)
}

fn clipboard_named_pipe_dir_path(dir_state: &impl DirectoryStateProvider) -> Result<PathBuf> {
    let path = dir_state.home_dir()?.join(".neovim-container");
    Ok(path)
}
