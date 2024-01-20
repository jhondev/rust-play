use std::io::stdin;

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
    let buffer = read_stdin("Enter some text:", &mut stdin().lock()); // Add the `stdin` reader as the second argument
    println!("{:?}", args);
    println!("{}", colors::red(&buffer));
}
