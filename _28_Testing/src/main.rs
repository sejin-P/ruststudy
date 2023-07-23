use std::fmt::format;

// 28.1 Unit Tests
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello World"), "Hello");
}

// 28.2 Test Modules
// Unit tests are often put in a nestued module.

fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}

// This lets you unit test private helpers
// The #[cfg(test)] attribute is only active when you run cargo test





// 28.3 Documentation Tests
// Rust has built-in support for documentation tests:

// Code blocks in /// comments are automatically seen as Rust code.



/// Shortens a string to the given length
///
/// ```
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}

fn main() {
    println!("Hello, world!");
}

