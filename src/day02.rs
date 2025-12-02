// https://adventofcode.com/2025/day/2

use fancy_regex::Regex;

static INPUT_FILE: &'static str = include_str!("../data/day02.txt");

fn solve(regex_str: &str) -> u64 {
    let re = Regex::new(&regex_str).expect("invalid regex");

    INPUT_FILE
        .trim()
        .split(",")
        .flat_map(|range_str| match range_str.split_once('-') {
            Some((start_str, end_str)) => {
                let start_id: u64 = start_str.parse().unwrap();
                let end_id: u64 = end_str.parse().unwrap();
                (start_id..=end_id)
                    .into_iter()
                    .map(|id| re.is_match(&id.to_string()).unwrap().then(|| id))
                    .flatten()
            }
            _ => panic!("malformed range: {}", range_str),
        })
        .sum()
}

pub fn part1() -> () {
    let sum = solve(r"^(\d+)\1$");
    println!("Day 02, Part 1");
    println!("{}", sum);
}

pub fn part2() -> () {
    let sum = solve(r"^(\d+)\1+$");
    println!("Day 02, Part 2");
    println!("{}", sum);
}
