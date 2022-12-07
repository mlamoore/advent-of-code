use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let calc_priority = |ch| {
        if ch >= 'a' as u8 {
            ch - 'a' as u8 + 1
        } else {
            ch - 'A' as u8 + 27
        }
    };

    input
        .lines()
        .map(|sack| {
            let sack = sack.as_bytes();
            let len = sack.len();
            let first = &sack[0..(len / 2)];
            let second = &sack[(len / 2)..];
            let mut priority = 0;
            for ch in first.iter() {
                if second.iter().any(|ch2| *ch == *ch2) {
                    priority = calc_priority(*ch);
                    break;
                }
            }
            priority as usize
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let calc_priority = |ch| {
        if ch >= 'a' as u8 {
            ch - 'a' as u8 + 1
        } else {
            ch - 'A' as u8 + 27
        }
    };

    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let first = group.next().unwrap().as_bytes();
            let second = group.next().unwrap().as_bytes();
            let third = group.next().unwrap().as_bytes();
            let mut priority = 0;
            for ch in first.iter() {
                if second.iter().any(|ch2| *ch == *ch2) && third.iter().any(|ch2| *ch == *ch2) {
                    priority = calc_priority(*ch);
                    break;
                }
            }
            priority as usize
        })
        .sum()
}
