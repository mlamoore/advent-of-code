use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<usize>>, aoc_parse::ParseError> {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(lines(digit+));

    p.parse(&input)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    let height = input.len();
    let width = input[0].len();

    input
        .iter()
        .enumerate()
        .map(|(y, rowvec)| {
            rowvec
                .iter()
                .enumerate()
                .filter(move |(x, tree)| {
                    input[y].iter().take(*x).all(|other| *tree > other)
                        || input[y]
                            .iter()
                            .rev()
                            .take(width - 1 - *x)
                            .all(|other| *tree > other)
                        || input.iter().take(y).all(|row| **tree > row[*x])
                        || input
                            .iter()
                            .rev()
                            .take(height - 1 - y)
                            .all(|row| **tree > row[*x])
                })
                .count()
        })
        .sum()
}

//#[aoc(day8, part2)]
pub fn solve_part2_iter(input: &[Vec<usize>]) -> usize {
    let height = input.len();
    let width = input[0].len();

    assert_eq!(input[height - 1].len(), width);

    input
        .iter()
        .enumerate()
        .map(|(y, rowvec)| {
            rowvec
                .iter()
                .enumerate()
                .filter(|&(x, _)| x != 0 && y != 0 && x != width - 1 && y != height - 1)
                .map(|(x, tree)| {
                    input[y]
                        .iter()
                        .take(x)
                        .rev()
                        .copied()
                        .enumerate()
                        .find(|(_, other)| *tree <= *other)
                        .map(|(num, _)| num + 1)
                        .unwrap_or(x)
                        * input[y]
                            .iter()
                            .rev()
                            .take(width - 1 - x)
                            .rev()
                            .copied()
                            .enumerate()
                            .find(|(_, other)| *tree <= *other)
                            .map(|(num, _)| num + 1)
                            .unwrap_or(width - 1 - x)
                        * input
                            .iter()
                            .take(y)
                            .map(|row| row[x])
                            .rev()
                            .enumerate()
                            .find(|(_, other)| *tree <= *other)
                            .map(|(num, _)| num + 1)
                            .unwrap_or(y)
                        * input
                            .iter()
                            .rev()
                            .take(height - 1 - y)
                            .map(|row| row[x])
                            .rev()
                            .enumerate()
                            .find(|(_, other)| *tree <= *other)
                            .map(|(num, _)| num + 1)
                            .unwrap_or(height - 1 - y)
                })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2_hybrid(input: &[Vec<usize>]) -> usize {
    let height = input.len();
    let width = input[0].len();

    assert_eq!(input[height - 1].len(), width);

    (1..(height - 1))
        .flat_map(|y| {
            (1..(width - 1)).map(move |x| {
                (1..)
                    .find(|num_trees_seen| {
                        let xt = x - num_trees_seen;
                        xt == 0 || input[y][xt] >= input[y][x]
                    })
                    .unwrap()
                    * (1..)
                        .find(|num_trees_seen| {
                            let xt = x + num_trees_seen;
                            xt == width - 1 || input[y][xt] >= input[y][x]
                        })
                        .unwrap()
                    * (1..)
                        .find(|num_trees_seen| {
                            let yt = y - num_trees_seen;
                            yt == 0 || input[yt][x] >= input[y][x]
                        })
                        .unwrap()
                    * (1..)
                        .find(|num_trees_seen| {
                            let yt = y + num_trees_seen;
                            yt == height - 1 || input[yt][x] >= input[y][x]
                        })
                        .unwrap()
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator(
            "\
30373
25512
65332
33549
35390",
        )
        .unwrap();
        assert_eq!(solve_part1(&input), 21);
    }
}
