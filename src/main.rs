mod cli;
mod container_runner;
mod non_pure;
mod path;

use std::fs;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use nix::sys::stat;
use nix::unistd::{self, getpid};
use non_pure::DirectoryStateProviderImpl;

use crate::cli::{Args, RawArgs};
use crate::container_runner::{run_container, DirectoryStateProvider};

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(short, long, default_value_t = false)]
//     develop: bool,
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
//     path: Option<String>,
// }

fn main() {
    // let path = Path::new("/home/vscode/hoge");
    // match unistd::mkfifo(path, stat::Mode::S_IRWXU) {
    //     Ok(_) => println!("created {:?}", path),
    //     Err(err) => println!("Error creating fifo: {}", err),
    // }
    // print_pid("second");
    // std::thread::spawn(move || loop {
    //     let mut file = fs::File::create(path).unwrap();
    //     file.write_all(b"hogehoge").unwrap();
    //     print_pid("spawned second");
    // });
    let args = Args::from(RawArgs::parse());
    println!("develop: {}", args.develop);
    println!("path: {:?}", args.path);
    println!(
        "absolute path: {:?}",
        args.path
            .map(|p| DirectoryStateProviderImpl.absolute_path(p))
    );

    // run_container(DirectoryStateProviderImpl);
    // print_pid("contianer finished");
}

fn print_pid(tag: &str) {
    println!("{}:{}", tag, getpid().as_raw());
}
