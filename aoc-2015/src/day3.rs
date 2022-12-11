use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub fn follow_directions(input: &str) -> HashMap<(i32, i32), usize> {
    let mut visited = HashMap::new();

    let (mut x, mut y) = (0i32, 0i32);

    *visited.entry((x, y)).or_insert(0) += 1;

    for dir in input.chars() {
        match dir {
            '^' => {
                y += 1;
                *visited.entry((x, y)).or_insert(0) += 1;
            },
            'v' => {
                y -= 1;
                *visited.entry((x, y)).or_insert(0) += 1;
            },
            '>' => {
                x += 1;
                *visited.entry((x, y)).or_insert(0) += 1;
            },
            '<' => {
                x -= 1;
                *visited.entry((x, y)).or_insert(0) += 1;
            },
            _ => panic!(),
        }
    }

    visited
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    follow_directions(input).len()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let santa = input.chars().step_by(2).collect::<String>();
    let robosanta = input.chars().skip(1).step_by(2).collect::<String>();

    let mut santa = follow_directions(&santa);
    let robosanta = follow_directions(&robosanta);

    for (key, val) in robosanta.iter() {
        *santa.entry(*key).or_insert(0) += *val;
    }

    santa.len()
}
