use clap::Parser;
use cli_utils::{colors, read_stdin};

#[derive(Parser)]
#[clap()]
#[derive(Debug)]
struct Args {
    // Define your command-line arguments here
}

fn main() {
    let args = Args::parse();
    let buffer = read_stdin("Enter some text:");
    println!("{:?}", args);
    println!("{}", colors::red(&buffer));
}
