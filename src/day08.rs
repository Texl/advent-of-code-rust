// https://adventofcode.com/2025/day/8

use glam::{I64Vec3, Vec3};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::vec;

static INPUT_FILE: &'static str = include_str!("../data/day08.txt");

fn parse_input(input: &str) -> Vec<I64Vec3> {
    let ret = input
        .lines()
        .map(
            |line| match line.split(',').map(|x| x.parse::<i64>().unwrap()) {
                mut nums => I64Vec3::new(
                    nums.next().unwrap(),
                    nums.next().unwrap(),
                    nums.next().unwrap(),
                ),
            },
        )
        .collect_vec();

    ret
}

fn get_closest_box_pairs(boxes: Vec<I64Vec3>) -> vec::IntoIter<(I64Vec3, I64Vec3, f32)> {
    boxes
        .to_vec()
        .into_iter()
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
}

fn connect_boxes(
    box_groups: &Vec<Vec<I64Vec3>>,
    box_a: &I64Vec3,
    box_b: &I64Vec3,
) -> Vec<Vec<I64Vec3>> {
    let group_a = box_groups
        .iter()
        .find(|g| g.contains(box_a))
        .expect("Group containing point a not found");

    let group_b = box_groups
        .iter()
        .find(|g| g.contains(box_b))
        .expect("Group containing point b not found");

    if !(group_a.eq(group_b)) {
        let mut next_box_groups = box_groups.to_vec();
        next_box_groups.retain(|g| !g.eq(group_a) && !g.eq(group_b));
        let mut merged = group_a.to_vec();
        merged.extend(group_b);
        next_box_groups.push(merged);
        next_box_groups
    } else {
        box_groups.clone()
    }
}

pub fn part1() -> () {
    let junction_boxes = parse_input(INPUT_FILE);

    let closest_junction_box_pairs = get_closest_box_pairs(junction_boxes.clone());

    let mut current_junction_box_groups = junction_boxes.into_iter().map(|v| vec![v]).collect_vec();

    for (a, b, _dist) in closest_junction_box_pairs.take(1000) {
        current_junction_box_groups = connect_boxes(&mut current_junction_box_groups, &a, &b);
    }

    let result: usize = current_junction_box_groups
        .iter()
        .map(|g| g.len())
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("Day 08, Part 1");
    println!("{:#?}", result);
}

pub fn part2() -> () {
    let junction_boxes = parse_input(INPUT_FILE);

    let closest_junction_box_pairs = get_closest_box_pairs(junction_boxes.clone());

    let mut current_junction_box_groups = junction_boxes.into_iter().map(|v| vec![v]).collect_vec();

    let (result, _) = closest_junction_box_pairs
        .clone()
        .fold_while(
            (0, current_junction_box_groups),
            |(_, box_groups), (a, b, _)| {
                let next_box_groups = connect_boxes(&box_groups, &a, &b);

                if next_box_groups.len() == 1 {
                    Done((a.x * b.x, next_box_groups))
                } else {
                    Continue((a.x * b.x, next_box_groups))
                }
            },
        )
        .into_inner();

    println!("Day 08, Part 2");
    println!("{:#?}", result);
}
