use std::{path::Path, process::Command};

pub fn run_container() {
    NvimCommandExecutor {
        image: "mijinko17/neovim-container:latest",
        volumes: vec![],
        work_dir: Path::new("/"),
    }
    .execute();
}

struct VolumeArg {
    host_path: String,
    container_path: String,
}

struct NvimCommandExecutor<T: AsRef<Path>> {
    image: &'static str,
    volumes: Vec<VolumeArg>,
    work_dir: T,
}

impl<T> NvimCommandExecutor<T>
where
    T: AsRef<Path>,
{
    pub fn execute(self) {
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
