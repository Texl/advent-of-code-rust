// https://adventofcode.com/2025/day/4

use num_traits::ToPrimitive;
use std::collections::HashSet;

static INPUT_FILE: &'static str = include_str!("../data/day04.txt");

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn get_roll_positions(grid: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    grid.iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.iter()
                .enumerate()
                .flat_map(|(c, ch)| match ch {
                    '@' => Some((r as i32, c as i32)),
                    _ => None,
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<HashSet<(i32, i32)>>()
}

fn get_char_at(grid: &Vec<Vec<char>>, r: i32, c: i32) -> Option<char> {
    match (r.to_usize(), c.to_usize()) {
        (None, _) | (_, None) => None,
        (Some(r), Some(c)) if r >= grid.len() || c >= grid[0].len() => None,
        (Some(r), Some(c)) => Some(grid[r][c]),
    }
}

fn get_neighbors(grid: &Vec<Vec<char>>, r: i32, c: i32) -> Vec<char> {
    let mut neighbors = Vec::new();
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            if let Some(ch) = get_char_at(grid, r + dr, c + dc) {
                neighbors.push(ch);
            }
        }
    }
    neighbors
}

fn get_removable(
    grid: &Vec<Vec<char>>,
    roll_positions: &HashSet<(i32, i32)>,
) -> HashSet<(i32, i32)> {
    roll_positions
        .into_iter()
        .filter(|pos| {
            let count = get_neighbors(&grid, pos.0, pos.1)
                .into_iter()
                .filter(|&ch| ch == '@')
                .count();
            count < 4
        })
        .copied()
        .collect::<HashSet<(i32, i32)>>()
}

fn remove_from_grid(grid: &mut Vec<Vec<char>>, removable: &HashSet<(i32, i32)>) -> () {
    for pos in removable {
        grid[pos.0 as usize][pos.1 as usize] = 'x';
    }
}

pub fn part1() -> () {
    let grid: Vec<Vec<_>> = parse_grid(INPUT_FILE);
    let roll_positions = get_roll_positions(&grid);
    let result = get_removable(&grid, &roll_positions).len();

    println!("Day 04, Part 1");
    println!("{}", result);
}

pub fn part2() -> () {
    let mut grid: Vec<Vec<_>> = parse_grid(INPUT_FILE);
    let mut roll_positions = get_roll_positions(&grid);
    let mut result = 0;

    loop {
        match get_removable(&grid, &roll_positions) {
            rem if rem.is_empty() => break,
            rem => {
                result += rem.len();
                remove_from_grid(&mut grid, &rem);
                roll_positions.retain(|e| !rem.contains(e));
            }
        }
    }

    println!("Day 04, Part 2");
    println!("{}", result);
}
