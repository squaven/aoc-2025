use std::fs::File;
use std::io::{BufRead, BufReader};

struct Dial {
    position: i32,
}

impl Dial {
    fn new(position: i32) -> Self {
        Dial { position }
    }

    fn rotate_right(&mut self, steps: i32) {
        println!("  DEBUG {0}", self.position);
        self.position = self.position + steps;
        println!("  DEBUG {0}", self.position);
        while self.position > 99 {
            self.position = self.position - 100;
            println!("    DEBUG {0}", self.position);
        }
    }

    fn rotate_left(&mut self, steps: i32) {
        println!("  DEBUG {0}", self.position);
        self.position = self.position - steps;
        println!("  DEBUG {0}", self.position);
        while self.position < 0 {
            self.position = self.position + 100;
            println!("    DEBUG {0}", self.position);
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
}

fn main() {
    let mut dial = Dial::new(50);

    let password_result =
        File::open("input.txt")
            .map(|file| BufReader::new(file))
            .map(|reader| {
                let mut password_counter = 0;
                for line in reader.lines() {
                    let line = line.unwrap();
                    let initial_dial = dial.get_position();
                    dial.rotate_from_line(&line);
                    if dial.get_position() == 0 {
                        password_counter = password_counter + 1;
                        println!("DEBUG {0} -> {1} -> {2} PASS {3}", initial_dial, line, dial.get_position(), password_counter);
                    }
                    else {
                        println!("DEBUG {0} -> {1} -> {2}", initial_dial, line, dial.get_position());
                    }
                    println!("")
                }
                password_counter
            });
    match password_result {
        Ok(p) => println!("Password {0}" , p.to_string()),
        Err(e) => println!("ERROR {0}" , e.to_string()),
    }
}
