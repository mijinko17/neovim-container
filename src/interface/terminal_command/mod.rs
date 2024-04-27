use std::{ffi::OsStr, process::Command};

pub mod get_win_clipboard_command;
pub mod pull_image_command;
pub mod run_from_string_command;
pub mod run_nvim_container_command;
pub mod set_win_clipboard_command;

pub trait OptionalArg {
    fn optional_arg<S: AsRef<OsStr>>(&mut self, optional_arg: Option<S>) -> &mut Self;
}

impl OptionalArg for Command {
    fn optional_arg<S: AsRef<OsStr>>(&mut self, optional_arg: Option<S>) -> &mut Self {
        match optional_arg {
            Some(value) => self.arg(value),
            None => self,
        }
    }
}
