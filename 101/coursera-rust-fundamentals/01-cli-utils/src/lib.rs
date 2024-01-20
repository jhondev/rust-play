//! This is a library that provides utilities for cli applications.

use std::io::BufRead;

pub mod colors;

/// Reads a line from the standard input (stdin) and returns it as a String.
///
/// # Arguments
///
/// * `msg` - A message to display before reading the input.
/// * `reader` - A mutable reference to a `BufRead` object.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use cli_utils::read_stdin;
///
/// let input = "Hello, World!\n";
/// let mut reader = Cursor::new(input);
/// let message = read_stdin("Enter a message: ", &mut reader);
/// println!("{}", message);
/// ```
///
/// # Panics
///
/// This function will panic if it fails to read a line from stdin.
///
/// # Returns
///
/// A String containing the line read from stdin, without the trailing newline character.
pub fn read_stdin<R: BufRead>(msg: &str, reader: &mut R) -> String {
    println!("{}", msg);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stdin() {
        // Create a mock reader with a predefined input
        let input = "Hello, World!\n";
        let mut reader = input.as_bytes();

        // Call the function and assert the result
        let result = read_stdin("Enter a message: ", &mut reader);
        assert_eq!(result, "Hello, World!");
    }
}
