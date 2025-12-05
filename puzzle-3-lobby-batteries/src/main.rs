use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Bank {
    // batteries: Vec<u32>
    batteries: String
}

impl Bank {
    // fn new(line: &str) -> Self {
    //     let mut batteries: Vec<u32> = vec![];
    //     for c in line.chars() {
    //         batteries.push(c.to_digit(10).unwrap())
    //     }
    //     Bank {
    //         batteries
    //     }
    // }

    fn new(line: &str) -> Self {
        Bank {
            batteries: line.to_string()
        }
    }

    fn list_to_value(list: Vec<usize>) -> usize {
        let mut acc = 0;
        let base: usize = 10;
        for i in 0..list.len() {
            acc = acc + list[i]* base.pow((list.len()-i-1) as u32)
        }
        acc
    }

    fn maximize(&self, digits: usize) -> usize {
        let mut indices: Vec<usize> = vec![];
        let mut values: Vec<usize> =  vec![];
        let mut current_index: usize;

        for d in 1..digits+1 {
            current_index = match indices.last() {
                | Some (p) => *p + 1,
                | None => 0
            };
            // println!("Digit {0}, Start {1}, End {2}", d, current_index, digits-d);
            let mut highest_value: u32 = 0;
            let mut highest_index: usize = 0;
            for i in current_index..self.batteries.len()-(digits-d) {
                // print!("Checking {0} ", i);
                let current_value = ((
                    self.batteries
                        .as_bytes()[i]) as char)
                        .to_digit(10)
                        .unwrap();
                // println!("Value {0}", current_value);
                if current_value > highest_value {
                    // println!("  Highest {0} ({1})", current_value, i);
                    highest_value = current_value;
                    highest_index = i;
                }
            };
            values.push(highest_value as usize);
            indices.push(highest_index);
            // println!("Pushing {0}", highest_value);
            // println!("")
        }

        Self::list_to_value(values)
    }

    // fn highest_joltage(&self) -> u32 {
    //     let mut highest_first_index: usize = 0;
    //     let mut highest_second_index: usize = 0;
    //     let mut highest: u32 = 0;
    //     for (i, &joltage) in self.batteries[0..self.batteries.len()-1].iter().enumerate() {
    //         if joltage > highest {
    //             highest = joltage;
    //             highest_first_index = i;
    //         }
    //     }
    //     highest = 0;
    //     for (i, &joltage) in self.batteries[highest_first_index+1..self.batteries.len()].iter().enumerate() {
    //         if joltage > highest {
    //             highest = joltage;
    //             highest_second_index = i+highest_first_index+1;
    //         }
    //     }
    //     (self.batteries[highest_first_index]*10) + self.batteries[highest_second_index]
    // }
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

    let highest_joltages =
        File::open(input_file)
            .map(|file| BufReader::new(file))
            .map(|reader| {
                let mut bank_vec: Vec<usize> = Vec::new();
                for line in reader.lines() {
                    let line = line.unwrap();
                    let bank = Bank::new(&line);
                    let highest = bank.maximize(12);
                    bank_vec.push(highest);
                    println!("Highest: {0}", highest)
                }
                bank_vec
            });
    match highest_joltages {
        | Ok(joltages) => {
            let sum: usize = joltages.iter().sum();
            println!("SUM {0}", sum);
        }
        | Err(e) => println!("{0}", e)
    }
}
