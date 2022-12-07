use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

pub fn fuel_used(start_pos: &[usize], finish: usize) -> usize {
    start_pos
        .iter()
        .map(|&s| if s > finish { s - finish } else { finish - s })
        .sum()
}

pub fn fuel_used2(start_pos: &[usize], finish: usize) -> usize {
    start_pos
        .iter()
        .map(|&s| if s > finish { s - finish } else { finish - s })
        .map(|s| s * (s + 1) / 2)
        .sum()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let max_pos: usize = *input.iter().max().unwrap();

    let mut low_fuel = usize::MAX;

    for finish in 0..max_pos {
        let fuel = fuel_used(input, finish);

        if fuel < low_fuel {
            low_fuel = fuel;
        }
    }

    low_fuel
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let max_pos: usize = *input.iter().max().unwrap();

    let mut low_fuel = usize::MAX;

    for finish in 0..max_pos {
        let fuel = fuel_used2(input, finish);

        if fuel < low_fuel {
            low_fuel = fuel;
        }
    }

    low_fuel
}
