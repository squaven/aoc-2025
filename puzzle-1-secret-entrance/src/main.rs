use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Dial {
    position: i32,
    zero_count: i32
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            zero_count: 0
        }
    }

    fn rotate_right(&mut self, steps: i32) {
        let mut initial_position = self.position.clone();
        self.position = self.position + steps;
        while self.position > 99 {
            self.position = self.position - 100;
            if initial_position != 0 && self.position != 0 {
                self.zero_count = self.zero_count + 1;
            }
            initial_position = self.position.clone();
        }
        if self.position == 0 {
            self.zero_count = self.zero_count + 1;
        }
    }

    fn rotate_left(&mut self, steps: i32) {
        let mut initial_position = self.position.clone();
        self.position = self.position - steps;
        while self.position < 0 {
            self.position = self.position + 100;
            if initial_position != 0 && self.position != 0 {
                self.zero_count = self.zero_count + 1;
            }
            initial_position = self.position.clone();
        }
        if self.position == 0 {
            self.zero_count = self.zero_count + 1
        }
    }

    fn rotate_from_line(&mut self, line: &str) {
        if line.chars().nth(0).unwrap() == 'R' {
            self.rotate_right(line[1..].parse::<i32>().unwrap());
        }
        else {
            self.rotate_left(line[1..].parse::<i32>().unwrap());
        }
    }

    fn get_position(&self) -> i32 {
        self.position
    }
    fn get_count(&self) -> i32 {
        self.zero_count
    }
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


    let mut dial = Dial::new();

    let password_result =
        File::open(input_file)
            .map(|file| BufReader::new(file))
            .map(|reader| {
                for line in reader.lines() {
                    let line = line.unwrap();
                    let initial_dial = dial.get_position();
                    dial.rotate_from_line(&line);
                    println!("DEBUG {0} -> {1} -> {2} PASS {3}", initial_dial, line, dial.get_position(), dial.get_count());
                }
                dial.get_count()
            });
    match password_result {
        Ok(p) => println!("Password {0}" , p.to_string()),
        Err(e) => println!("ERROR {0}" , e.to_string()),
    }
}
