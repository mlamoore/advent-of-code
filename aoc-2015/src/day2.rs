use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split('x')
                .into_iter()
                .map(|section| section.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .map(|p| (p[0], p[1], p[2]))
        .map(|(l, w, h)| (l * w, l * h, w * h))
        .map(|(a, b, c)| a.min(b).min(c) + 2 * a + 2 * b + 2 * c)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .map(|p| (p[0], p[1], p[2]))
        .map(|(l, w, h)| (2 * l + 2 * w, 2 * l + 2 * h, 2 * w + 2 * h, l * w * h))
        .map(|(a, b, c, v)| a.min(b).min(c) + v)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example_generator() {
        let input = "2x3x4\n1x1x10";
        let input = input_generator(input);

        assert_eq!(input, vec![vec![2, 3, 4], vec![1, 1, 10]]);
    }

    #[test]
    fn test_p1_example() {
        let input = "2x3x4\n1x1x10";
        let input = input_generator(input);

        assert_eq!(solve_part1(&input), 58 + 43);
    }
}
