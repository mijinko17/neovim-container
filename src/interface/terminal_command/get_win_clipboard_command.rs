use std::process::Command;

use anyhow::Result;

pub struct GetWindowsClipboardCommand;

impl GetWindowsClipboardCommand {
    pub fn execute(&self) -> Result<String> {
        let powershell = Command::new("powershell.exe")
            .arg("-command")
            .arg("[Console]::OutputEncoding = [System.Text.Encoding]::GetEncoding('utf-8');Get-Clipboard")
            .output()?;
        let mut result = String::from_utf8(powershell.stdout)?.replace("\r\n", "\n");
        if result.ends_with('\n') {
            result.pop();
        }
        Ok(result)
    }
}
