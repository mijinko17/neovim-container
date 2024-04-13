use std::path::PathBuf;

use anyhow::Result;

use crate::{
    clipboard::clipboard_named_pipe_dir_path, directory_state::DirectoryStateProvider,
    terminal_command::set_host_clipboard_command_executor::SetHostClipboardCommandExecutor,
};

pub fn setup_clipboard_from_container(
    dir_state: &'static impl DirectoryStateProvider,
) -> Result<()> {
    let _ = create_named_pipes(dir_state);
    listen_and_set_clipboard(dir_state)
}

fn create_named_pipes(dir_state: &'static impl DirectoryStateProvider) -> Result<()> {
    let _ = dir_state.create_directory(&clipboard_named_pipe_dir_path(dir_state)?);
    dir_state.create_named_pipes(&clipboard_named_pipe_from_container_path(dir_state)?)
}

fn listen_and_set_clipboard(dir_state: &'static impl DirectoryStateProvider) -> Result<()> {
    let path = clipboard_named_pipe_from_container_path(dir_state)?;
    std::thread::spawn(move || loop {
        let content = dir_state.file_content(&path).unwrap();
        let _ = set_clipboard(content);
    });
    Ok(())
}

pub fn clipboard_named_pipe_from_container_path(
    dir_state: &impl DirectoryStateProvider,
) -> Result<PathBuf> {
    let path = clipboard_named_pipe_dir_path(dir_state)?.join("from_container");
    Ok(path)
}

fn set_clipboard(content: String) -> Result<()> {
    SetHostClipboardCommandExecutor::new(content).execute()
}
