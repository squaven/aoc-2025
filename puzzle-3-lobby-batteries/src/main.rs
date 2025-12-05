use std::fs::File;
use std::io::{BufRead, BufReader};

struct Bank {
    batteries: Vec<u32>
}

impl Bank {
    fn new(line: &str) -> Self {
        let mut batteries: Vec<u32> = vec![];
        for c in line.chars() {
            batteries.push(c.to_digit(10).unwrap())
        }
        Bank {
            batteries
        }
    }

    fn highest_joltage(&self) -> u32 {
        let mut highest_first_index: usize = 0;
        let mut highest_second_index: usize = 0;
        let mut highest: u32 = 0;
        for (i, &joltage) in self.batteries[0..self.batteries.len()-1].iter().enumerate() {
            if joltage > highest {
                highest = joltage;
                highest_first_index = i;
            }
        }
        highest = 0;
        for (i, &joltage) in self.batteries[highest_first_index+1..self.batteries.len()].iter().enumerate() {
            if joltage > highest {
                highest = joltage;
                highest_second_index = i+highest_first_index+1;
            }
        }
        (self.batteries[highest_first_index]*10) + self.batteries[highest_second_index]
    }
}

fn main() {
    let highest_joltages =
        File::open("input.txt")
            .map(|file| BufReader::new(file))
            .map(|reader| {
                let mut bank_vec: Vec<u32> = Vec::new();
                for line in reader.lines() {
                    let line = line.unwrap();
                    let bank = Bank::new(&line);
                    let highest = bank.highest_joltage();
                    bank_vec.push(highest);
                    println!("Highest: {0}", highest)
                }
                bank_vec
            });
    match highest_joltages {
        | Ok(joltages) => {
            let sum: u32 = joltages.iter().sum();
            println!("SUM {0}", sum);
        }
        | Err(e) => println!("{0}", e)
    }
}
