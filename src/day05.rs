// https://adventofcode.com/2025/day/5

use itertools::Itertools;

static INPUT_FILE: &'static str = include_str!("../data/day05.txt");

fn split_input(lines: &[&str]) -> (Vec<String>, Vec<String>) {
    let (index, _) = lines.iter().find_position(|s| str::is_empty(s)).unwrap();
    match lines.split_at(index) {
        (range_strs, product_strs) => (
            range_strs
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            product_strs[1..].iter().map(|s| s.to_string()).collect(),
        ),
    }
}

fn parse_ranges(range_strs: &Vec<String>) -> Vec<(i64, i64)> {
    range_strs
        .iter()
        .map(|s| {
            let mut parts = s.split("-").map(|p| p.trim());
            let start = parts.next().unwrap().parse::<i64>().unwrap();
            let end = parts.next().unwrap().parse::<i64>().unwrap();

            (start, end)
        })
        .collect::<Vec<(i64, i64)>>()
}

fn parse_ingredients(ingredient_strs: &[String]) -> Vec<i64> {
    ingredient_strs
        .iter()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn part1() -> () {
    let lines: Vec<_> = INPUT_FILE.trim().split("\n").collect();

    let (range_strs, product_strs) = split_input(&lines);

    let ingredient_ranges = parse_ranges(&range_strs);

    let ingredients = parse_ingredients(&product_strs[1..]);

    let result = ingredients
        .iter()
        .filter(|&ingredient| {
            ingredient_ranges
                .iter()
                .any(|(start, end)| *ingredient >= *start && *ingredient <= *end)
        })
        .collect::<Vec<&i64>>()
        .len();

    println!("Day 05, Part 1");
    println!("{}", result)
}

pub fn part2() -> () {
    let lines: Vec<_> = INPUT_FILE.trim().split("\n").collect();

    let (range_strs, _) = split_input(&lines);

    let ingredient_ranges = parse_ranges(&range_strs);

    let coalesced_ranges =
        ingredient_ranges
            .iter()
            .sorted()
            .fold(vec![], |mut acc: Vec<(i64, i64)>, &next| {
                if let Some(last) = acc.last_mut() {
                    if next.0 <= last.1 {
                        last.1 = i64::max(last.1, next.1);
                    } else {
                        acc.push(next);
                    }
                } else {
                    acc.push(next);
                }
                acc
            });

    let result = coalesced_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<i64>();

    println!("Day 05, Part 2");
    println!("{}", result)
}
