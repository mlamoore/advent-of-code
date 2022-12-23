use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CpuInst {
    AddX(i32),
    NoOp,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Vec<CpuInst>, aoc_parse::ParseError> {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(lines({"noop" => CpuInst::NoOp, "addx " n: i32 => CpuInst::AddX(n)}));

    p.parse(&input)
}

pub fn execute_instructions(instructions: &[CpuInst]) -> Vec<i32> {
    let mut output = Vec::new();
    let mut register = 1;

    for inst in instructions {
        match inst {
            CpuInst::NoOp => output.push(register),
            CpuInst::AddX(x) => {
                output.push(register);
                output.push(register);
                register += x;
            }
        }
    }

    output
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[CpuInst]) -> i32 {
    let values = execute_instructions(input);

    (0..6)
        .map(|i| 20 + 40 * i)
        .map(|i| values[i - 1] * i as i32)
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[CpuInst]) -> String {
    let values = execute_instructions(input);

    let mut output = String::with_capacity(248);
    output.push('\n');

    for y in 0..6 {
        for x in 0..40 {
            let cycle = y * 40 + x;

            output.push(if (x as i32 - values[cycle]).abs() <= 1 {
                '#'
            } else {
                '.'
            });
        }

        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        )
        .unwrap();
        assert_eq!(solve_part1(&input), 13140);
    }
}
