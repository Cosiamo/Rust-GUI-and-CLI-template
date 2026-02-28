use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Author Name", version, about)]
pub struct Commands {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    #[command(about = "Add two numbers together")]
    Add(Args),
    #[command(about = "Subtract two numbers")]
    Subtract(Args),
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'a', long = "left")]
    pub left_number: f64,
    #[arg(short = 'b', long = "right")]
    pub right_number: f64,
}
