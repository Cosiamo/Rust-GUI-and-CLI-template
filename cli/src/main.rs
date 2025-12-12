use std::{error::Error, process::Command};

pub mod my_file;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        println!("Args greater than 1");
        my_file::my_function();
    } else {
        println!("Args less than 1");
        Command::new(r"gui-bindings.exe")
            .args(args)
            .spawn()?
            .wait()?;
    }

    Ok(())
}
