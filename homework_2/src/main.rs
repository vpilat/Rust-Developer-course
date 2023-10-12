//! Homework from Lesson 2 - Rust Basics: Syntax and Variables
/// Rust Programming Code Example

use std::io;
use std::env;
extern crate slug;
use slug::slugify;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[0]);

    let mut input = String::new();
    println!("Enter string please...");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Original: {}", input);

    let slug = slugify(input);
    println!("Slugified: {}", slug);
}
