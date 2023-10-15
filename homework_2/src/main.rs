//! Homework from Lesson 2 - Rust Basics: Syntax and Variables
//! 
//! use: CLI param  lowercase - convert string to lowercase
//!                 upercase  - convert string to upercase
//!                 no_spaces - convert string to string with no spaces
//!                 slugify   - convert string to lowercase with - instead of spaces
//! 
/// Rust Programming Code Example

use std::io;
use std::env;
extern crate slug;
use slug::slugify;

fn main() {
    let args: Vec<String> = env::args().collect();                                      // init var, store cli args

    println!("arg: {}", args[1]);                                                       // print cli args

    let mut input = String::new();                                              // init var
    
    println!("Enter source text, please...");                                           // print message (ask for string)

    io::stdin().read_line(&mut input).expect("Failed to read text");           // read string from stdio

    print!("Input : {}", input);                                                        // print input string

    print!("Output: ");

    match args[1].as_str() {                                                            // match string manipulations method and print manipulated string
        "lowercase" => println!("{}", input.to_lowercase()),
        "upercase" => println!("{}", input.to_uppercase()),
        "no_spaces" => println!("{}", input.replace(" ", "")),
        "slugify" => println!("{}", slugify(input)),
        _ => println!("unsupported method..."),
    }
}
