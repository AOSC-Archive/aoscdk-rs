use frontend::{Args, execute};
use clap::Parser;

mod disks;
mod frontend;
mod install;
mod network;
mod parser;

fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        frontend::tui_main();
    } else {
        let args = Args::parse();
        execute(args).unwrap();
    }
}
