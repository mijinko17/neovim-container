use std::process::Command;

pub fn run_container() {
    Command::new("vi").spawn().unwrap().wait().unwrap();
}
