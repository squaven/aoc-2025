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

    fn rotate_clicks_right(&mut self, steps: i32) {
        print!("{0}: R{1}\n  ", self.position, steps);
        for _ in 0..steps {
            self.position = self.position + 1;
            print!("{0}", self.position);
            while self.position > 99 {
                self.position = self.position - 100;
                print!("W{0}", self.position);
            }
            if self.position == 0 {
                print!("HIT");
                self.zero_count = self.zero_count + 1;
                print!("{0}", self.position);
            }
            print!(" ");
        }
        println!("COUNT: {0}", self.zero_count);
    }

    fn rotate_clicks_left(&mut self, steps: i32) {
        print!("{0}: L{1}\n  ", self.position, steps);
        for _ in 0..steps {
            self.position = self.position - 1;
            print!("{0}", self.position);
            if self.position < 0 {
                self.position = self.position + 100;
                print!("W{0}", self.position);
            }
            if self.position == 0 {
                print!("HIT");
                self.zero_count = self.zero_count + 1;
            }
            print!(" ");
        }
        println!("\nCOUNT: {0}", self.zero_count);
    }

    fn rotate_from_line(&mut self, line: &str) {
        if line.chars().nth(0).unwrap() == 'R' {
            self.rotate_clicks_right(line[1..].parse::<i32>().unwrap());
        }
        else {
            self.rotate_clicks_left(line[1..].parse::<i32>().unwrap());
        }
    }

    fn get_count(&self) -> i32 {
        self.zero_count
    }
}

fn main() {
    let mut dial = Dial::new();

    let password_result =
        File::open("input.txt")
            .map(|file| BufReader::new(file))
            .map(|reader| {
                for line in reader.lines() {
                    let line = line.unwrap();
                    dial.rotate_from_line(&line);
                    println!("");
                }
                dial.get_count()
            });
    match password_result {
        Ok(p) => println!("Password {0}" , p.to_string()),
        Err(e) => println!("ERROR {0}" , e.to_string()),
    }
}
