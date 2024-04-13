use std::path::PathBuf;

use anyhow::Result;

use crate::{
    directory_state::DirectoryStateProvider,
    terminal_command::get_clipboard_win::GetWindowsClipboardCommandExecutor,
};

pub fn setup_clipboard_from_host(dir_state: &'static impl DirectoryStateProvider) -> Result<()> {
    let _ = create_named_pipes(dir_state);
    listen_and_send_clipboard_from_host(dir_state)
}

fn create_named_pipes(dir_state: &impl DirectoryStateProvider) -> Result<()> {
    dir_state.create_directory(&clipboard_named_pipe_dir_path(dir_state)?)?;
    dir_state.create_named_pipes(&clipboard_named_pipe_path(dir_state)?)
}

fn listen_and_send_clipboard_from_host(
    dir_state: &'static impl DirectoryStateProvider,
) -> Result<()> {
    let path = clipboard_named_pipe_path(dir_state)?;
    std::thread::spawn(move || loop {
        let _ = dir_state.file_content(&path).unwrap();
        let _ = dir_state.write_file(&path, clipboard_content().unwrap().as_ref());
    });
    Ok(())
}

pub fn clipboard_named_pipe_path(dir_state: &impl DirectoryStateProvider) -> Result<PathBuf> {
    let path = clipboard_named_pipe_dir_path(dir_state)?.join("clipboard");
    Ok(path)
}

fn clipboard_named_pipe_dir_path(dir_state: &impl DirectoryStateProvider) -> Result<PathBuf> {
    let path = dir_state.home_dir()?.join(".neovim-container");
    Ok(path)
}

fn clipboard_content() -> Result<Vec<u8>> {
    let a = GetWindowsClipboardCommandExecutor
        .execute()?
        .as_bytes()
        .to_vec();
    Ok(a)
}
