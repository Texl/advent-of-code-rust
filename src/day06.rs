// https://adventofcode.com/2025/day/6

use crate::util;
use itertools::Itertools;

static INPUT_FILE: &'static str = include_str!("../data/day06.txt");

enum Operator {
    Add,
    Mul,
}

fn parse_operators(operators_line: &str) -> Vec<Operator> {
    operators_line
        .split_whitespace()
        .map(|ch| match ch {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => panic!("Unknown operator"),
        })
        .collect()
}

fn split_input(lines: Vec<&str>) -> Option<(String, Vec<String>)> {
    let max_line_length = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let padded_lines = lines
        .iter()
        .map(|line| format!("{:max_line_length$}", line))
        .collect::<Vec<String>>();

    match padded_lines.split_last() {
        Some((operators_line, operand_lines)) => {
            Some((operators_line.clone(), operand_lines.to_vec()))
        }
        None => None,
    }
}

fn apply(operand_lists: Vec<Vec<i64>>, operator_list: Vec<Operator>) -> i64 {
    operand_lists
        .iter()
        .zip(operator_list.iter())
        .map(|(operands, operator)| match operator {
            Operator::Add => operands.iter().sum::<i64>(),
            Operator::Mul => operands.iter().product(),
        })
        .sum::<i64>()
}

pub fn part1() -> () {
    let lines: Vec<_> = INPUT_FILE.trim().split("\n").collect();

    let (operators_line, operand_lines) = split_input(lines).unwrap();

    let operand_lists: Vec<Vec<i64>> = operand_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let operator_list = parse_operators(&operators_line);

    let result = apply(util::transpose(operand_lists), operator_list);

    println!("Day 06, Part 1");
    println!("{}", result);
}

pub fn part2() -> () {
    let lines: Vec<_> = INPUT_FILE.trim().split("\n").collect();

    let (operators_line, operand_lines) = split_input(lines).unwrap();

    let operand_lists_0 = operand_lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let operand_lists = util::transpose(operand_lists_0)
        .into_iter()
        .map(|chars| {
            chars
                .into_iter()
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .ok()
        })
        .chunk_by(|x| x.is_some())
        .into_iter()
        .map(|(k, ch)| ch.flatten().collect::<Vec<i64>>())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let operator_list = parse_operators(&operators_line);

    let result = apply(operand_lists, operator_list);

    println!("Day 06, Part 2");
    println!("{}", result);
}
