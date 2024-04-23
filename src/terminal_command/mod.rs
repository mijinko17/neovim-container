use std::{ffi::OsStr, process::Command};

pub mod get_clipboard_win;
pub mod pull_image_command;
pub mod run_nvim_container_command;
pub mod set_host_clipboard_command_executor;

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
