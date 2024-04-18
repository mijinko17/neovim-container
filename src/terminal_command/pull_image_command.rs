use std::process::Command;

use anyhow::Result;

pub struct PullImageComand {
    pub image: String,
}

impl PullImageComand {
    pub fn new(image: String) -> Self {
        Self { image }
    }
    pub fn execute(self) -> Result<()> {
        Command::new("docker")
            .arg("pull")
            .arg(self.image)
            .spawn()?
            .wait()?;
        Ok(())
    }
}
