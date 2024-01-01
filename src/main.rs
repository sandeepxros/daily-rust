#![allow(unused)]

use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
fn main() {
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice To Meet You";
    io::stdin()
        .read_line(&mut name)
        .expect("You dind't enterd your name");

    println!("Hello, {},  {}", name.trim_end(), greeting);
}
