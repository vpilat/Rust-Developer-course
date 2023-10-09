//! Homework from Lesson 2
/// Rust Programming Code Example

use std::io::stdin;
// extern crate slug;
// use slug::slugify;

fn main() {
    println!();
    print!("Enter text: ");
    
    // Get the text from the keyboard
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    // Slugify the text.
    // let slug = text.trim().to_lowercase().replace(" ", "-");

    // Print the slug to the console.
    println!();
    println!("{}", line);
    println!();
}
