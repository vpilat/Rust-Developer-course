//! Homework from Lesson 2
/// Rust Programming Code Example

use std::io;
extern crate slug;
use slug::slugify;

fn main() {
    // Create a new instance of stdin
    let stdin = io::stdin();

    // Create a mutable string to store the input
    let mut input = String::new();

    // Read a line from stdin and store it in the 'input' string
    stdin
        .read_line(&mut input)
        .expect("Failed to read line from stdin");

    let slug = slugify("Hello world");

    // Print the line back out
    println!("You entered: {}", input);
}
