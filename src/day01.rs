// https://adventofcode.com/2025/day/1

use once_cell::sync::Lazy;
use super::math;

static INPUT_FILE: &'static str = include_str!("../data/day01.txt");

struct Rotation(i32);

struct Position(i32);

const DIAL_SIZE: i32 = 100;
const INITIAL_STATE: Position = Position(50);
const ROTATIONS: Lazy<Vec<Rotation>> = Lazy::new(parse_rotations);

fn parse_rotations() -> Vec<Rotation> {
    fn parse_rotation(str: &str) -> Rotation {
        let (direction, ticks_str) = str.split_at(1);
        match (direction, ticks_str.parse::<i32>()) {
            ("L", Ok(ticks)) => Rotation(-ticks),
            ("R", Ok(ticks)) => Rotation(ticks),
            _ => panic!("malformed rotation: {}", str),
        }
    }

    INPUT_FILE
        .trim()
        .split("\n")
        .map(parse_rotation)
        .collect::<Vec<_>>()
}

fn apply_rotation(rotation: &Rotation, position: &Position) -> Position {
    Position(math::modulo(position.0 + rotation.0, DIAL_SIZE))
}

pub fn part1() -> () {
    let count: i32 = ROTATIONS
        .iter()
        .scan(INITIAL_STATE, |position, rotation| {
            *position = apply_rotation(rotation, position);
            let is_zero_landing = position.0 == 0;
            Some(i32::from(is_zero_landing))
        })
        .sum();

    println!("Day 01, Part 1");
    println!("{}", count)
}

pub fn part2() -> () {
    let count: i32 = ROTATIONS
        .iter()
        .scan(INITIAL_STATE, |position, rotation| {
            let zero_dist = math::modulo(rotation.0.signum() * position.0, DIAL_SIZE);
            let zero_landings = (zero_dist + rotation.0.abs()) / DIAL_SIZE;
            *position = apply_rotation(rotation, position);
            Some(zero_landings)
        })
        .sum();

    println!("Day 01, Part 2");
    println!("{}", count)
}
