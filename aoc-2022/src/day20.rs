use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<i64> {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(lines(i64));

    p.parse(&input).unwrap()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mixed = mix(input, 1);

    let zero_pos = mixed
        .iter()
        .enumerate()
        .find(|(_i, x)| **x == 0)
        .map(|(i, _x)| i)
        .unwrap();

    let len = mixed.len();
    mixed[(zero_pos + 1000) % len] + mixed[(zero_pos + 2000) % len] + mixed[(zero_pos + 3000) % len]
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let input = input.iter().map(|x| *x * 811589153).collect::<Vec<_>>();

    let mixed = mix(&input, 10);

    let zero_pos = mixed
        .iter()
        .enumerate()
        .find(|(_i, x)| **x == 0)
        .map(|(i, _x)| i)
        .unwrap();

    let len = mixed.len();
    mixed[(zero_pos + 1000) % len] + mixed[(zero_pos + 2000) % len] + mixed[(zero_pos + 3000) % len]
}

fn mix(orig: &[i64], repeats: usize) -> Vec<i64> {
    let len = orig.len();

    let mut positions = (0..len).into_iter().collect::<Vec<usize>>();

    //println!("Initial arrangement:\n{:?}\n", positions.iter().map(|pos| orig[*pos]).collect::<Vec<_>>());

    for _ in 0..repeats {
        for i in 0..len {
            let this_val = orig[i];
            let this_pos = positions[i];
            let new_pos = (this_pos as i64 + this_val).rem_euclid((len - 1) as i64) as usize;

            let (min_shift, max_shift, shift_dir) = if new_pos > this_pos {
                (this_pos + 1, new_pos, -1)
            } else if new_pos < this_pos {
                (new_pos, this_pos - 1, 1)
            } else {
                //println!("{} does not move:\n{:?}\n", orig[i], positions.iter().map(|pos| orig[*pos]).collect::<Vec<_>>());
                continue;
            };

            for j in 0..len {
                if positions[j] >= min_shift && positions[j] <= max_shift {
                    positions[j] = (positions[j] as i64 + shift_dir) as usize;
                }
            }

            positions[i] = new_pos;

            let mut check_coverage = positions.clone();
            check_coverage.sort();
            assert!(check_coverage.iter().enumerate().all(|(i, j)| i == *j));

            // Visualization code

            let mut output = vec![0; len];

            for i in 0..len {
                output[positions[i]] = orig[i];
            }

            //println!("Moved {} from {} to {} (shift {}-{} by {})\n{:?}\n", orig[i], this_pos, new_pos, min_shift, max_shift, shift_dir, output);

            // End visualizations
        }
    }

    let mut output = vec![0; len];

    for i in 0..len {
        output[positions[i]] = orig[i];
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator("1\n2\n-3\n3\n-2\n0\n4");
        assert_eq!(solve_part1(&input), 3);
    }

    #[test]
    fn test_part1_mix_find() {
        let input = input_generator("1\n2\n-3\n3\n-2\n0\n4");
        let mixed = mix(&input, 1);

        let zero_pos = mixed
            .iter()
            .enumerate()
            .find(|(_i, x)| **x == 0)
            .map(|(i, _x)| i)
            .unwrap();

        let len = mixed.len();
        assert_eq!(
            (
                mixed[(zero_pos + 1000) % len],
                mixed[(zero_pos + 2000) % len],
                mixed[(zero_pos + 3000) % len]
            ),
            (4, -3, 2)
        );
    }

    #[test]
    fn test_part1_modulo() {
        let input = input_generator("61\n62\n-63\n63\n-62\n0\n64");
        assert_eq!(solve_part1(&input), 64 - 63 + 62);
    }
}
