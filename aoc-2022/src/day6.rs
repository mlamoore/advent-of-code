use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .chars()
        .zip(input.chars().skip(1))
        .zip(input.chars().skip(2))
        .zip(input.chars().skip(3))
        .enumerate()
        .filter(|(_, (((a, b), c), d))| a != b && a != c && a != d && b != c && b != d && c != d)
        .next()
        .unwrap()
        .0
        + 4
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .filter(|(pos, _)| input.chars().skip(*pos).take(14).unique().count() == 14)
        .next()
        .unwrap()
        .0
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
