use std::process::Command;

pub fn run_container() {
    Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--interactive")
        .arg("--tty")
        .args(vec!["--workdir", "/"])
        .arg("--network=host")
        .arg("mijinko17/neovim-container:latest")
        .arg("nvim")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
