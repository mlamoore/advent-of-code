use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CargoAction {
    num_crates: usize,
    src: usize,  // 0-indexed, problem/input is 1-indexed
    dest: usize, // 0-indexed, problem/input is 1-indexed
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<CargoAction>) {
    let mut stacks = Vec::new();
    let mut instructions = Vec::new();
    let mut finished_stacks = false;

    for line in input.lines() {
        if !finished_stacks {
            for (i, ch) in line.chars().enumerate() {
                if i & 3 == 1 {
                    let stack = (i - 1) / 4; // 0 indexed vs problem 1-indexed
                    if ch.is_numeric() {
                        // Reached the end of the stacks
                        finished_stacks = true;
                        break;
                    } else if ch.is_alphabetic() {
                        while stacks.len() <= stack {
                            stacks.push(Vec::new());
                        }
                        stacks[stack].insert(0, ch);
                    }
                }
            }
        } else {
            let mut inst_parts = line.split(' ');
            let inst_start = inst_parts.next();
            if inst_start.is_some() {
                if "move" == inst_start.unwrap() {
                    // This isn't the blank line, it's a move instruction
                    let num_crates = inst_parts.next().unwrap().parse().unwrap();
                    let _ = inst_parts.next();
                    let src = inst_parts.next().unwrap().parse::<usize>().unwrap() - 1; // Go from 1-based to 0-based
                    let _ = inst_parts.next();
                    let dest = inst_parts.next().unwrap().parse::<usize>().unwrap() - 1; // Go from 1-based to 0-based

                    instructions.push(CargoAction {
                        num_crates,
                        src,
                        dest,
                    });
                }
            }
        }
    }

    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<Vec<char>>, Vec<CargoAction>)) -> String {
    let mut stacks = input.0.clone();
    let instructions = &input.1;

    for inst in instructions {
        for _ in 0..inst.num_crates {
            let current = stacks[inst.src].pop().unwrap();
            stacks[inst.dest].push(current);
        }
    }

    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Vec<Vec<char>>, Vec<CargoAction>)) -> String {
    let mut stacks = input.0.clone();
    let instructions = &input.1;

    for inst in instructions {
        // add crates to new location
        let bot_moved = stacks[inst.src].len() - inst.num_crates;
        for i in 0..inst.num_crates {
            let current = stacks[inst.src][bot_moved + i];
            stacks[inst.dest].push(current);
        }
        for _ in 0..inst.num_crates {
            let _ = stacks[inst.src].pop();
        }
    }

    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let example = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let desired_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        let desired_instructions = vec![
            CargoAction {
                num_crates: 1,
                src: 1,
                dest: 0,
            },
            CargoAction {
                num_crates: 3,
                src: 0,
                dest: 2,
            },
            CargoAction {
                num_crates: 2,
                src: 1,
                dest: 0,
            },
            CargoAction {
                num_crates: 1,
                src: 0,
                dest: 1,
            },
        ];

        let (parsed_stacks, parsed_instructions) = input_generator(example);

        assert_eq!(desired_stacks, parsed_stacks);
        assert_eq!(desired_instructions, parsed_instructions);
    }
}
