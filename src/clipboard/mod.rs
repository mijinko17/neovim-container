pub mod from_container;
pub mod from_host;

use std::path::PathBuf;

use anyhow::Result;

use crate::interface::directory_state::DirectoryStateProvider;

use self::{
    from_container::{clipboard_named_pipe_from_container_path, setup_clipboard_from_container},
    from_host::{clipboard_named_pipe_from_host_path, setup_clipboard_from_host},
};

pub fn setup_clipboard(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    setup_clipboard_from_host(dir_state, container_name)?;
    setup_clipboard_from_container(dir_state, container_name)
}

pub fn clean_named_pipe(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    let _ = dir_state.remove_file(&clipboard_named_pipe_from_container_path(
        dir_state,
        container_name,
    )?);
    let _ = dir_state.remove_file(&clipboard_named_pipe_from_host_path(
        dir_state,
        container_name,
    )?);
    Ok(())
}

fn clipboard_named_pipe_dir_path(dir_state: &impl DirectoryStateProvider) -> Result<PathBuf> {
    let path = dir_state.home_dir()?.join(".neovim-container");
    Ok(path)
}
