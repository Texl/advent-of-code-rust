// https://adventofcode.com/2025/day/7

use itertools::Itertools;

static INPUT_FILE: &'static str = include_str!("../data/day07.txt");

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<char>>) {
    match input.lines().collect_vec().split_first() {
        Some((first_line, remaining_lines)) => {
            let initial_state = first_line
                .chars()
                .map(|ch| if ch == 'S' { 1 } else { 0 })
                .collect();

            let grid = remaining_lines
                .into_iter()
                .map(|line| line.chars().collect_vec())
                .collect();

            (initial_state, grid)
        }
        None => panic!("Input is empty"),
    }
}

fn is_splitter(ch: &char) -> bool {
    *ch == '^'
}

pub fn part1() -> () {
    fn count_splits(state: Vec<i64>, manifold_row: &Vec<char>) -> (i64, Vec<i64>) {
        let mut splits: i64 = 0;
        let mut next_state: Vec<i64> = state.clone();

        for splitter_position in manifold_row.iter().positions(is_splitter) {
            if state[splitter_position] > 0 {
                splits += 1;
                next_state[splitter_position] = 0;
            }

            let mut split_to = |position: usize| -> () {
                next_state.get_mut(position).iter_mut().for_each(|split| {
                    if **split == 0 {
                        **split = 1
                    }
                });
            };

            split_to(splitter_position - 1);
            split_to(splitter_position + 1);
        }

        (splits, next_state)
    }

    let (initial_state, grid) = parse_input(INPUT_FILE);

    let result = grid
        .iter()
        .fold((0, initial_state), |(total_splits, beams), row| {
            let (splits, new_beams) = count_splits(beams, row);
            (total_splits + splits, new_beams)
        })
        .0;

    println!("Day 07, Part 1");
    println!("{:?}", result);
}

pub fn part2() -> () {
    fn step_beams(state: Vec<i64>, manifold_row: &Vec<char>) -> Vec<i64> {
        let mut next_state = state.to_vec();

        for splitter_position in manifold_row.iter().positions(is_splitter) {
            next_state[splitter_position] = 0;

            let mut split_to = |position: usize| -> () {
                next_state
                    .get_mut(position)
                    .iter_mut()
                    .for_each(|count| **count += state[splitter_position]);
            };

            split_to(splitter_position - 1);
            split_to(splitter_position + 1);
        }

        next_state
    }

    let (initial_state, grid) = parse_input(INPUT_FILE);

    let result: i64 = grid.iter().fold(initial_state, step_beams).iter().sum();

    println!("Day 07, Part 2");
    println!("{:?}", result);
}
