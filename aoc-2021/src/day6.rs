use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut ages = [0usize; 9];

    for f in input {
        ages[*f] += 1;
    }

    for _day in 0..80 {
        let spawns = ages[0];

        for i in 1..9 {
            ages[i - 1] = ages[i];
        }

        ages[8] = spawns;
        ages[6] += spawns;
    }

    ages.iter().sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut ages = [0usize; 9];

    for f in input {
        ages[*f] += 1;
    }

    for _day in 0..256 {
        let spawns = ages[0];

        for i in 1..9 {
            ages[i - 1] = ages[i];
        }

        ages[8] = spawns;
        ages[6] += spawns;
    }

    ages.iter().sum()
}
