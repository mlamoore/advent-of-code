use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    let mut group = Vec::new();
    let mut current = Vec::new();

    for line in input.lines().map(|l| l.trim().parse()) {
        match line {
            Ok(cal) => current.push(cal),
            Err(_) => group.push(std::mem::replace(&mut current, Vec::new())),
        }
    }

    if !current.is_empty() {
        group.push(current);
    }

    group
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .map(|elf| elf.iter().sum())
        .fold(0, |sum, next: usize| next.max(sum))
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<usize>]) -> usize {
    let mut elves = input
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<usize>>();

    elves.sort();

    elves.iter().rev().take(3).sum()
}
