use crate::util;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| util::bin_str_to_num(l.trim()).unwrap())
        .collect()
}

pub fn bit_counts(input: &[usize]) -> [usize; 12] {
    let mut ones = [0usize; 12];

    for digit in 0..12 {
        for i in 0..input.len() {
            if input[i] & (1 << digit) != 0 {
                ones[digit] += 1;
            }
        }
    }

    ones
}

pub fn bits_to_num(input: [usize; 12], threshold: usize) -> usize {
    let mut gamma = 0;

    for bit in 0..12 {
        if input[bit] > threshold {
            gamma += 1 << bit;
        }
    }

    gamma
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let ones = bit_counts(input);

    let gamma = bits_to_num(ones, input.len() / 2);

    let epsilon = (1 << 12) - 1 - gamma;

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut oxy = input.to_vec();

    let mut pos = 11;

    let oxygen = loop {
        let ones = bit_counts(&oxy);

        let want_zero = ones[pos] < oxy.len() - ones[pos];

        oxy = oxy
            .iter()
            .filter(|n| ((**n & (1 << pos)) == 0) == want_zero)
            .map(|n| *n)
            .collect();

        if oxy.len() == 1 {
            break oxy[0];
        }

        pos = if pos > 0 { pos - 1 } else { 11 };
    };

    let mut co2 = input.to_vec();

    let mut pos = 11;

    let co2 = loop {
        let ones = bit_counts(&co2);

        let want_zero = ones[pos] >= co2.len() - ones[pos];

        co2 = co2
            .iter()
            .filter(|n| ((**n & (1 << pos)) == 0) == want_zero)
            .map(|n| *n)
            .collect();

        if co2.len() == 1 {
            break co2[0];
        }

        pos = if pos > 0 { pos - 1 } else { 11 };
    };

    oxygen * co2
}
