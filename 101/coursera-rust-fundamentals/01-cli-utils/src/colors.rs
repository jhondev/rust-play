//! ANSI color codes for use in the terminal

/// Returns a string with the ANSI color code for red
/// # Examples
/// ```
/// use cli_utils::colors::red;
/// println!("{}", red("Hello, world!"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}
