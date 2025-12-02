// Input is a series of rotations
// Starts with either L (left, toward lower numbers) or R (right, toward higher numbers), depending on which direction to turn
// Next is a distance value, which indicates how many clicks the dial should be rotated
// There is an overflow, i.e. turning 1 to the left from 0 lands on 99 and vice versa
// The password is the number of times the dial is left pointing at 0 after any rotation in the sequence
// The dial starts by pointing at 50

use std::{
    fs::File,
    i32,
    io::{BufRead, BufReader},
};

fn main() {
    let input = read_input();
    let mut dial_number = 50;
    let mut zeroes = 0;

    for mut instruction in input {
        let direction = match instruction.remove(0) {
            'L' => -1,
            'R' => 1,
            _ => panic!("Could not read direction in {:?}!", instruction),
        };

        let clicks = instruction.parse::<i32>().unwrap();

        dial_number = turn_dial(dial_number, clicks, direction);

        if dial_number == 0 {
            zeroes += 1
        }
    }

    println!("This many 0s were counted: {:?}", zeroes)
}

fn turn_dial(dial_number: i32, steps: i32, direction: i32) -> i32 {
    let result = match dial_number + steps * direction {
        n @ i32::MIN..=-1 => {
            if steps > 100 {
                n + ((steps / 100) * 100)
            } else {
                n + 100
            }
        }
        n @ 0..=99 => n,
        n @ 100..=i32::MAX => {
            if steps > 100 {
                n - ((steps / 100) * 100)
            } else {
                n - 100
            }
        }
    };

    result
}

fn read_input() -> Vec<String> {
    // Read file
    let file = match File::open("input") {
        Ok(value) => value,
        Err(e) => {
            panic!("{:?}", e);
        }
    };

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];

    for line in reader.lines() {
        let r = match line {
            Ok(val) => val,
            Err(e) => {
                println!("{:?}", e);
                String::from("")
            }
        };

        lines.push(r);
    }

    lines
}
