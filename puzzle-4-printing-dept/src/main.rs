use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file =  {
        if args.len() > 1 {
            if args[1] == "-e" { "example.txt" }
            else { "input.txt" }
        }
        else { "input.txt" }
    };

    println!("Hello, world!");
}
