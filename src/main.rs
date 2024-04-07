mod container_runner;
mod non_pure;
mod path;

use std::fs;
use std::io::Write;
use std::path::Path;

use nix::sys::stat;
use nix::unistd::{self, getpid};
use non_pure::DirectoryStateProviderImpl;

use crate::container_runner::run_container;

fn main() {
    let path = Path::new("/home/vscode/hoge");
    match unistd::mkfifo(path, stat::Mode::S_IRWXU) {
        Ok(_) => println!("created {:?}", path),
        Err(err) => println!("Error creating fifo: {}", err),
    }
    print_pid("second");
    std::thread::spawn(move || loop {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(b"hogehoge").unwrap();
        print_pid("spawned second");
    });
    run_container(DirectoryStateProviderImpl);
    print_pid("contianer finished");
}

fn print_pid(tag: &str) {
    println!("{}:{}", tag, getpid().as_raw());
}
