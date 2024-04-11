mod cli;
mod command_executor;
mod constants;
mod container_config;
mod container_runner;
mod directory_state;
mod path;
mod update_binary;

use clap::Parser;
use directory_state::DirectoryStateProviderImpl;
use update_binary::update_binary;

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
    if args.update {
        update_binary().expect("Failed to update binary.")
    } else {
        run_container(args, DirectoryStateProviderImpl)
    }
}
