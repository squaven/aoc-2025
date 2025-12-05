use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Id {
    value: usize
}

impl Id {
    fn new(value: usize) -> Self {
        Id {
            value: value,
        }
    }

    fn is_valid_repeating(&self) -> bool {
        let value_string = self.value.to_string();
        let mut valid = true;
        // For every possible split across the length of the value
        'outer: for i in 2..value_string.len()+1 {
            // If evenly divisble
            if value_string.len() % i == 0 {
                let sub_len = value_string.len() / i;
                let mut prev = "previous";
                let mut cur = &value_string[..];
                while !cur.is_empty() {
                    // Divide into substring
                    let (chunk, rest) = cur.split_at(cmp::min(sub_len, cur.len()));
                    // Compare substring to previous substring. If any not match, then must be valid.
                    if prev != "previous" && chunk != prev { continue 'outer; }
                    prev = chunk;
                    cur = rest;
                }
                // If divisible and none don't match, then false.
                valid = false;
                break;
            }
        }
        valid
    }

    // fn is_valid_double(&self) -> bool {
    //     let value_string = self.value.to_string();
    //     let value_length = value_string.len();
    //     if value_length % 2 == 0 {
    //         let sub_length = value_length / 2;
    //         let first = value_string[0..sub_length].to_string();
    //         let second = value_string[sub_length..value_length].to_string();
    //         if first == second {
    //             return false;
    //         }
    //     }
    //     true
    // }

    fn get_value(&self) -> usize {
        self.value
    }
}

fn ids_from_boundaries(boundaries_str: &str) -> Vec<Id> {
    let boundaries = boundaries_str.split('-').collect::<Vec<&str>>();
    let mut return_vec = Vec::new();
    if boundaries.len() == 2 {
        let first = boundaries[0].parse::<usize>().unwrap();
        let second = boundaries[1].parse::<usize>().unwrap();
        for value in first..(second + 1) {
            return_vec.push(Id::new(value));
        }
    }
    return return_vec;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file =  {
        if args.len() > 1 {
            if args[1] == "-e" { "example.txt" }
            else { "input.txt" }
        }
        else { "input.txt" }
    };

    let id_list =
        File::open(input_file)
            .map(|file| BufReader::new(file))
            .map(|reader| {
                let mut id_vec: Vec<Id> = Vec::new();
                for line in reader.lines() {
                    let line = line.unwrap();
                    for bound in line.split(',') {
                        id_vec.append(&mut ids_from_boundaries(bound));
                    }
                }
                id_vec
            });
    match id_list {
        Ok(ids) => {
            let count =
                ids
                    .into_iter()
                    .fold(0, |acc, id| {
                        //if ! id.is_valid_double() {
                        if ! id.is_valid_repeating() {
                            println!("  BAD {0}", id.get_value());
                            acc + id.get_value()
                        }
                        else { acc }
                    });
            println!("TOTAL {0}" , count.to_string());
        },
        Err(e) => println!("ERROR {0}" , e.to_string()),
    }
}
