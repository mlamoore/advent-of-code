use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<Vec<bool>>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let mut input = input.to_owned();
    input.push('\n');

    let pair = parser!(usize "," usize);

    let p = parser!(lines(repeat_sep(pair, " -> ")));

    let walls = p.parse(&input).unwrap();

    let max_x = walls
        .iter()
        .flat_map(|section| section.iter().map(|(x, _y)| x))
        .max()
        .unwrap();
    let max_y = walls
        .iter()
        .flat_map(|section| section.iter().map(|(_x, y)| y))
        .max()
        .unwrap();

    let mut output = vec![vec![false; max_x + 200]; max_y + 3];

    for section in walls.iter() {
        for (&(x1, y1), &(x2, y2)) in section.iter().tuple_windows() {
            if x1 == x2 {
                let x = x1;
                for y in y1.min(y2)..=y1.max(y2) {
                    output[y][x] = true;
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    let y = y1;
                    output[y][x] = true;
                }
            }
        }
    }

    output
}

pub fn sand_fall(map: &mut [Vec<bool>]) -> usize {
    let mut amount = 0;

    'all_sand_grains: loop {
        let (mut x, mut y) = (500, 0);

        'single_grain: loop {
            if y + 1 >= map.len() {
                // This sand fell off the edge
                break 'all_sand_grains;
            } else if !map[y + 1][x] {
                y += 1;
            } else if !map[y + 1][x - 1] {
                y += 1;
                x -= 1;
            } else if !map[y + 1][x + 1] {
                y += 1;
                x += 1;
            } else {
                // The sand stops here
                map[y][x] = true;
                amount += 1;
                if x == 500 && y == 0 {
                    // We stopped at the sand source
                    break 'all_sand_grains;
                } else {
                    break 'single_grain;
                }
            }
        }
    }

    amount
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> usize {
    let mut map = input.to_vec();

    sand_fall(&mut map)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> usize {
    let mut map = input.to_vec();

    let floor_y = map.len() - 1;

    for x in 0..map[0].len() {
        map[floor_y][x] = true;
    }

    sand_fall(&mut map)
}
