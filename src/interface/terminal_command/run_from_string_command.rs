use std::process::Command;

pub struct RunFromStringCommand {
    command: String,
}

impl RunFromStringCommand {
    pub fn new(command: String) -> Self {
        Self { command }
    }
    pub fn execute(self) -> anyhow::Result<()> {
        Command::new("sh")
            .arg("-c")
            .arg(self.command)
            .spawn()?
            .wait()?;
        Ok(())
    }
}
