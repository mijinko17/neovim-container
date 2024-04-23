use std::path::PathBuf;

use anyhow::Result;

use crate::{
    clipboard::clipboard_named_pipe_dir_path,
    interface::{
        directory_state::DirectoryStateProvider,
        terminal_command::set_win_clipboard_command::SetWindowsClipboardCommand,
    },
};

pub fn setup_clipboard_from_container(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    let _ = create_named_pipes(dir_state, container_name);
    listen_and_set_clipboard(dir_state, container_name)
}

fn create_named_pipes(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    let _ = dir_state.create_directory(&clipboard_named_pipe_dir_path(dir_state)?);
    dir_state.create_named_pipes(&clipboard_named_pipe_from_container_path(
        dir_state,
        container_name,
    )?)
}

fn listen_and_set_clipboard(
    dir_state: &'static impl DirectoryStateProvider,
    container_name: &str,
) -> Result<()> {
    let path = clipboard_named_pipe_from_container_path(dir_state, container_name)?;
    std::thread::spawn(move || loop {
        let content = dir_state.file_content(&path).unwrap();
        let _ = set_clipboard(content);
    });
    Ok(())
}

pub fn clipboard_named_pipe_from_container_path(
    dir_state: &impl DirectoryStateProvider,
    container_name: &str,
) -> Result<PathBuf> {
    let path = clipboard_named_pipe_dir_path(dir_state)?
        .join(format!("clipboard_from_container_{}", container_name));
    Ok(path)
}

fn set_clipboard(content: String) -> Result<()> {
    SetWindowsClipboardCommand::new(content).execute()
}
