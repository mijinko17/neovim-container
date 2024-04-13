use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;

pub struct NvimCommandExecutor<T: AsRef<Path>, U: AsRef<Path>> {
    pub image: String,
    pub volumes: Vec<VolumeArg>,
    pub work_dir: T,
    pub target_file_path: Option<U>,
}

impl<T, U> NvimCommandExecutor<T, U>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    pub fn execute(self) -> Result<()> {
        Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--interactive")
            .arg("--tty")
            .args(vec!["--workdir", self.work_dir.as_ref().to_str().unwrap()])
            .arg("--network=host")
            .args(
                self.volumes
                    .into_iter()
                    .flat_map(|arg| arg.raw_volume_arg()),
            )
            .arg(self.image)
            .arg("nvim")
            .optional_arg(
                self.target_file_path
                    .and_then(move |p| p.as_ref().to_str().map(|s| s.to_string())),
            )
            .spawn()?
            .wait()?;
        Ok(())
    }
}

pub struct VolumeArg {
    host_path: PathBuf,
    container_path: PathBuf,
}

impl VolumeArg {
    pub fn new(host_path: impl AsRef<Path>, container_path: impl AsRef<Path>) -> VolumeArg {
        VolumeArg {
            host_path: host_path.as_ref().to_path_buf(),
            container_path: container_path.as_ref().to_path_buf(),
        }
    }
    pub fn raw_volume_arg(self) -> Vec<String> {
        vec![
            "--volume".to_string(),
            format!(
                "{}:{}",
                self.host_path.to_str().unwrap(),
                self.container_path.to_str().unwrap()
            ),
        ]
    }
}

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
