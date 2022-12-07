use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(first, second)| first < second)
        .count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let windowed: Vec<usize> = input
        .iter()
        .zip(input.iter().skip(1).zip(input.iter().skip(2)))
        .map(|(first, (second, third))| first + second + third)
        .collect();

    solve_part1(&windowed)
}
