use crate::{
    args::{Commands, Subcommands},
    spawn::spawn_gui,
};
use clap::Parser;
use std::error::Error;

pub mod args;
pub mod spawn;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let commands = Commands::parse();

        match commands.cmd {
            Subcommands::Add(add) => {
                let res = app_core::equations::add(add.left_number, add.right_number);
                println!("{}", res);
            }
            Subcommands::Subtract(sub) => {
                let res = app_core::equations::subtract(sub.left_number, sub.right_number);
                println!("{}", res);
            }
        }
    } else {
        match spawn_gui(args) {
            Ok(run) => run,
            Err(e) => {
                eprintln!("Failed to wait for instance: {}", e);
                std::process::exit(1);
            }
        };
    }

    Ok(())
}
