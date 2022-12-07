use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(String, isize)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.trim().split(' ');
            (
                it.next().unwrap().to_string(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(String, isize)]) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;

    for (dir, distance) in input {
        match dir.as_str() {
            "forward" => {
                x += distance;
            }
            "down" => {
                y += distance;
            }
            "up" => {
                y -= distance;
            }
            _ => panic!(),
        }
    }

    x * y
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(String, isize)]) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut aim: isize = 0;

    for (dir, distance) in input {
        match dir.as_str() {
            "forward" => {
                x += distance;
                y += aim * distance;
            }
            "down" => {
                aim += distance;
            }
            "up" => {
                aim -= distance;
            }
            _ => panic!(),
        }
    }

    x * y
}
