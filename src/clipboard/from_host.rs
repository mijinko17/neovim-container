use std::path::PathBuf;

use anyhow::Result;

use crate::interface::{
    directory_state::DirectoryStateProvider,
    terminal_command::get_win_clipboard_command::GetWindowsClipboardCommand,
};

use super::clipboard_named_pipe_dir_path;

pub fn setup_clipboard_from_host(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    let _ = create_named_pipes(dir_state, container_name);
    listen_and_send_clipboard_from_host(dir_state, container_name)
}

fn create_named_pipes(dir_state: &impl DirectoryStateProvider, container_name: &str) -> Result<()> {
    let _ = dir_state.create_directory(&clipboard_named_pipe_dir_path(dir_state)?);
    dir_state.create_named_pipes(&clipboard_named_pipe_from_host_path(
        dir_state,
        container_name,
    )?)
}

fn listen_and_send_clipboard_from_host(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    let path = clipboard_named_pipe_from_host_path(dir_state, container_name)?;
    std::thread::spawn(move || loop {
        let _ = dir_state.file_content(&path).unwrap();
        let _ = dir_state.write_file(&path, clipboard_content().unwrap().as_ref());
    });
    Ok(())
}

pub fn clipboard_named_pipe_from_host_path(
    dir_state: &impl DirectoryStateProvider,
    container_name: &str,
) -> Result<PathBuf> {
    let path = clipboard_named_pipe_dir_path(dir_state)?
        .join(format!("clipboard_from_host_{}", container_name));
    Ok(path)
}

fn clipboard_content() -> Result<Vec<u8>> {
    let a = GetWindowsClipboardCommand.execute()?.as_bytes().to_vec();
    Ok(a)
}
