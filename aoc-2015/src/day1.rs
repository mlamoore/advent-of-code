use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> isize {
    input
        .as_bytes()
        .into_iter()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2_for_loop(input: &str) -> usize {
    let mut floor = 0;

    for (pos, dir) in input
        .trim()
        .as_bytes()
        .into_iter()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => panic!("Invalid character"),
        })
        .enumerate()
    {
        if floor < 0 {
            return pos;
        }

        floor += dir;
    }

    panic!("Never got to basement");
}

// This didn't work, because the return inside of fold returns from the closure, not solve_part2_fold...
pub fn solve_part2_fold(input: &str) -> isize {
    input
        .trim()
        .as_bytes()
        .into_iter()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => panic!("Invalid character"),
        })
        .enumerate()
        .fold(0isize, |floor, (pos, dir)| {
            if floor < 0 {
                return pos as isize; // Oops! This doesn't do what I initially expected!
            } else {
                floor + dir
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2_for_loop() {
        assert_eq!(solve_part2_for_loop("()())("), 5);
        assert_eq!(solve_part2_for_loop("()()())("), 7);
        assert_eq!(solve_part2_for_loop("(())))("), 5);
        assert_eq!(solve_part2_for_loop("((()))))((((("), 7);
        assert_eq!(solve_part2_for_loop("((()()))))((((("), 9);
    }
}
