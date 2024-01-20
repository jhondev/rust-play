//! This is a library that provides utilities for cli applications.

use std::io::{BufRead, BufReader};

pub mod colors;

/// Reads a line from the standard input (stdin) and returns it as a String.
/// # Examples
/// ```
/// let input = read_stdin();
/// println!("{}", input);
/// ```
///
/// # Panics
///
/// This function will panic if it fails to read a line from stdin.
///
/// # Returns
///
/// A String containing the line read from stdin, without the trailing newline character.mentation for read_stdin
pub fn read_stdin(msg: &str) -> String {
    println!("{}", msg);
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}
