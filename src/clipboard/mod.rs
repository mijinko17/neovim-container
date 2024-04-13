mod from_host;

use anyhow::Result;

use crate::directory_state::DirectoryStateProvider;

use self::from_host::setup_clipboard_from_host;

pub fn setup_clipboard(dir_state: &'static impl DirectoryStateProvider) -> Result<()> {
    setup_clipboard_from_host(dir_state)
}
