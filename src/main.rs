mod cli;
mod container_runner;
mod non_pure;
mod path;

use clap::Parser;
use non_pure::DirectoryStateProviderImpl;

use crate::cli::{Args, RawArgs};
use crate::container_runner::run_container;

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
    run_container(args, DirectoryStateProviderImpl);
}
