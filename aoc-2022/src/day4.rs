use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .into_iter()
                .flat_map(|elf| {
                    elf.split('-')
                        .into_iter()
                        .map(|section| section.parse::<usize>().unwrap())
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|pair| {
            (pair[0] <= pair[2] && pair[1] >= pair[3]) || (pair[2] <= pair[0] && pair[3] >= pair[1])
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|pair| {
            (pair[0] <= pair[2] && pair[1] >= pair[2])
                || (pair[0] <= pair[3] && pair[1] >= pair[3])
                || (pair[2] <= pair[0] && pair[3] >= pair[0])
                || (pair[2] <= pair[1] && pair[3] >= pair[1])
        })
        .count()
}
