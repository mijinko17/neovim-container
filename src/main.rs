use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::stat;
use nix::unistd::{self, getpid};

fn main() {
    println!("Hello, world!");
    let path = Path::new("/home/vscode/hoge");
    print_pid("first");
    match unistd::mkfifo(path, stat::Mode::S_IRWXU) {
        Ok(_) => println!("created {:?}", path),
        Err(err) => println!("Error creating fifo: {}", err),
    }
    print_pid("second");
    std::thread::spawn(move || loop {
        print_pid("spawned first");
        let mut file = fs::File::create(path).unwrap();
        file.write_all(b"hogehoge").unwrap();
        print_pid("spawned second");
    });
    print_pid("third");
    let mut vi = Command::new("vi").spawn().unwrap();
    vi.wait().unwrap();
    print_pid("third");
    sleep(Duration::from_secs(3));
    print_pid("fourth");
}

fn print_pid(tag: &str) {
    println!("{}:{}", tag, getpid().as_raw());
}
