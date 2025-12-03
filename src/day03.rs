// https://adventofcode.com/2025/day/3

use once_cell::sync::Lazy;
use std::str::FromStr;

static INPUT_FILE: &'static str = include_str!("../data/day03.txt");

const BANKS: Lazy<Vec<Vec<u64>>> = Lazy::new(parse_banks);

fn parse_banks() -> Vec<Vec<u64>> {
    fn parse_bank(line: &str) -> Vec<u64> {
        line.chars()
            .map(|d| u64::from_str(d.to_string().as_str()).unwrap())
            .collect::<Vec<_>>()
    }

    INPUT_FILE
        .trim()
        .split("\n")
        .map(parse_bank)
        .collect::<Vec<_>>()
}

fn get_largest_joltage(bank: &Vec<u64>, num_digits: usize, acc: u64) -> u64 {
    match num_digits {
        0 => acc,
        _ => {
            let (max_digit_index, max_digit) = bank
                .iter()
                .enumerate()
                .take(bank.len() - (num_digits - 1))
                .rev()
                .max_by_key(|elt| elt.1)
                .unwrap();

            get_largest_joltage(
                &bank
                    .into_iter()
                    .skip(max_digit_index + 1)
                    .copied()
                    .collect::<Vec<_>>(),
                num_digits - 1,
                10 * acc + max_digit,
            )
        }
    }
}

fn get_total_joltage(num_batteries: usize) -> u64 {
    BANKS
        .iter()
        .map(|bank| get_largest_joltage(bank, num_batteries, 0))
        .sum::<u64>()
}

pub fn part1() -> () {
    let total_joltage = get_total_joltage(2);
    println!("Day 03, Part 1");
    println!("{}", total_joltage);
}

pub fn part2() -> () {
    let total_joltage = get_total_joltage(12);
    println!("Day 03, Part 2");
    println!("{}", total_joltage);
}
