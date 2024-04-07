use std::{
    path::{Path, PathBuf},
    process::Command,
};

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
        Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--interactive")
            .arg("--tty")
            .args(vec!["--workdir", self.work_dir.as_str()])
            .arg("--network=host")
            .args(
                self.volumes
                    .into_iter()
                    .flat_map(|arg| {
                        vec![
                            "--volumes".to_string(),
                            format!("{}:{}", arg.host_path, arg.container_path),
                        ]
                    })
                    .collect::<Vec<_>>(),
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
    T: AsRef<Path>,
{
    fn is_ancestor_of(&self, maybe_child: T) -> bool;
    fn relative_path_from_ancsestor(&self, maybe_ancsestor: T) -> Option<PathBuf>;
}

impl<T, U> PathUtils<T> for U
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    fn is_ancestor_of(&self, maybe_child: T) -> bool {
        let self_path_buf = self.as_ref();
        maybe_child
            .as_ref()
            .to_path_buf()
            .ancestors()
            .into_iter()
            .any(|path| self_path_buf.eq(path))
    }

    fn relative_path_from_ancsestor(&self, maybe_ancsestor: T) -> Option<PathBuf> {
        if maybe_ancsestor.is_ancestor_of(self) {
            diff_paths(self, maybe_ancsestor)
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
        let a = Path::new("/home/yuki");
        let b: PathBuf = Path::new("/home/yuki/program").into();
        assert!(a.is_ancestor_of(b));
        let c: PathBuf = Path::new("/home/yuki").into();
        let d = Path::new("/home/hogem");
        assert!(!c.is_ancestor_of(d));
    }

    #[test]
    fn test_relative_path_from_ancsestor() {
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/home/yuki/program/hoge/fuga").into();
        assert_eq!(
            b.relative_path_from_ancsestor(a),
            Some(Path::new("program/hoge/fuga").to_path_buf())
        );
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/home/yuki-nagato/program/hoge/fuga").into();
        assert_eq!(b.relative_path_from_ancsestor(a), None);
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/home/poyo").into();
        assert_eq!(b.relative_path_from_ancsestor(a), None);
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/monyo").into();
        assert_eq!(b.relative_path_from_ancsestor(a), None);
    }
}
