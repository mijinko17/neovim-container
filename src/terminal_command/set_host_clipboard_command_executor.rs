use std::process::{Command, Stdio};

use anyhow::Result;

pub struct SetHostClipboardCommandExecutor {
    content: String,
}

impl SetHostClipboardCommandExecutor {
    pub fn new(content: String) -> Self {
        Self { content }
    }
    pub fn execute(self) -> Result<()> {
        let echo = Command::new("printf")
            .arg(self.content)
            .stdout(Stdio::piped())
            .spawn()?;
        let iconv = Command::new("iconv")
            .arg("-t")
            .arg("utf16")
            .stdin(Stdio::from(echo.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        Command::new("clip.exe")
            .stdin(Stdio::from(iconv.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(())
    }
}
