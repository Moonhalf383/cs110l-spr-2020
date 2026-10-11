use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = File::open(filename).unwrap();
    let mut num_of_words = 0;
    let mut num_of_lines = 0;
    let mut num_of_characters = 0;
    for line in io::BufReader::new(file).lines() {
        num_of_lines += 1;
        for word in line.unwrap().split_whitespace() {
            num_of_words += 1;
            num_of_characters += word.len();
        }
    }
    println!(
        "{} {} {} {}",
        num_of_words, num_of_lines, num_of_characters, filename
    );
}
