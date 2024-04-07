use std::{env::current_exe, path::PathBuf, process::Command};

use dirs::home_dir;
use pathdiff::diff_paths;

pub fn run_container() {
    NvimCommandExecutor {
        image: "mijinko17/neovim-container:latest",
        volumes: vec![],
        work_dir: "/".to_string(),
    }
    .execute();
}

struct VolumeArg {
    host_path: String,
    container_path: String,
}

struct NvimCommandExecutor {
    image: &'static str,
    volumes: Vec<VolumeArg>,
    work_dir: String,
}

impl NvimCommandExecutor {
    pub fn execute(self) {
        self.volumes
            .into_iter()
            .fold(
                Command::new("docker")
                    .arg("run")
                    .arg("--rm")
                    .arg("--interactive")
                    .arg("--tty")
                    .args(vec!["--workdir", self.work_dir.as_str()])
                    .arg("--network=host"),
                |acc, cur| {
                    acc.args(vec![
                        "--volumes",
                        format!("{}:{}", cur.host_path, cur.container_path).as_str(),
                    ])
                },
            )
            .arg(self.image)
            .arg("nvim")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

trait PathUtils<T>
where
    T: Into<PathBuf> + Clone,
{
    fn is_ancestor_of(&self, maybe_child: &T) -> bool;
    fn relative_path_from(&self, maybe_ancsestor: &T) -> Option<PathBuf>;
}

impl<T, U> PathUtils<T> for U
where
    T: Into<PathBuf> + Clone,
    U: Into<PathBuf> + Clone,
{
    fn is_ancestor_of(&self, maybe_child: &T) -> bool {
        let self_path_buf: PathBuf = self.clone().into();
        maybe_child
            .clone()
            .into()
            .ancestors()
            .into_iter()
            .any(|path| self_path_buf.eq(path))
    }

    fn relative_path_from(&self, maybe_ancsestor: &T) -> Option<PathBuf> {
        if maybe_ancsestor.is_ancestor_of(&self.clone().into()) {
            diff_paths(self.clone(), maybe_ancsestor)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::PathUtils;

    #[test]
    fn another() {
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/home/yuki/program").into();
        assert!(a.is_ancestor_of(b));
        let c: PathBuf = Path::new("/home/yuki").into();
        let d: PathBuf = Path::new("/home/hogem").into();
        assert!(!c.is_ancestor_of(d));
    }
}
