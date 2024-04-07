use std::process::Command;

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
                |acc, cur| acc.args(vec![cur.host_path, cur.container_path]),
            )
            .arg(self.image)
            .arg("nvim")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
