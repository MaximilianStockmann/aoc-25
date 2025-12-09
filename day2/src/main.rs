// Goal: Identify invalid product IDs within ranges
// Input: Product ID ranges as comma-seperated entries in a line
// Format: [Range start]-[Range end]
// Invalid IDs are any IDs which is made only of some sequence of digits repeated twice
// None of the numbers have leading zeroes
// Output: The sum of invalid product ID within the input ranges

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let ranges = read_input();
    let mut invalid_id_sum = 0_u64;

    for range in ranges {
        let invalid_ids = get_invalid_ids(range);

        invalid_id_sum += invalid_ids.iter().sum::<u64>();
    }

    println!("Sum: {:?}", invalid_id_sum);
}

fn get_invalid_ids(range: Range) -> Vec<u64> {
    let mut invalid_ids = vec![];

    for num in range.start..=range.end {
        // convenient shorthand for digit count
        let digit_count = num.clone().checked_ilog10().unwrap_or(0) + 1;

        if digit_count % 2 == 0 {
            if is_invalid_id(num, digit_count) {
                invalid_ids.push(num);
            }
        }
    }

    invalid_ids
}

fn is_invalid_id(num: u64, digit_count: u32) -> bool {
    let num = num.to_string();

    // let mut sequences_to_check: Vec<String> = vec![];

    let (first_half, second_half) = num.split_at((digit_count / 2).try_into().unwrap());

    // println!(
    //     "First half: {:?} | Second half: {:?}",
    //     first_half, second_half
    // );

    if first_half.eq(second_half) {
        return true;
    }

    false

    // for i in (1..=digit_count / 2).rev() {
    //     let num_copy = num.clone();
    //     let (first_half, _) = num_copy.split_at(i.try_into().unwrap());
    //     sequences_to_check.push(first_half.to_string());

    // }
}

fn read_input() -> Vec<Range> {
    let mut input_ranges: Vec<Range> = vec![];
    // Read file
    let file = match File::open("input") {
        Ok(value) => value,
        Err(e) => {
            panic!("{:?}", e);
        }
    };

    let file_reader = BufReader::new(file);

    let ranges_byte_it = file_reader.split(b',').map(|l| l.unwrap());

    for range in ranges_byte_it {
        let input_range = String::from_utf8(range.to_vec()).unwrap();
        let mut input_range_it = input_range.split('-');
        let input_range = Range {
            start: input_range_it.next().unwrap().parse::<u64>().unwrap(),
            end: input_range_it.next().unwrap().parse::<u64>().unwrap(),
        };

        input_ranges.push(input_range);
    }

    input_ranges
}
