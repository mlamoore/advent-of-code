use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListItem {
    Num(i32),
    List(Vec<ListItem>),
}

type Input = Vec<(String, String)>;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(sections(
        line(string(any_char+))
        line(string(any_char+))
    ));

    p.parse(&input).unwrap()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut score = 0;

    for (index, (left, right)) in input.iter().enumerate() {
        if compare_p1(&left, &right, 0) == std::cmp::Ordering::Less {
            score += index + 1;
        }
    }

    score
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut packets = vec!["[[2]]", "[[6]]"];

    for (index, (left, right)) in input.iter().enumerate() {
        packets.push(left);
        packets.push(right);
    }

    packets.sort_by(|left, right| compare_p1(left, right, 0));

    let mut key = 1;

    for (i, packet) in packets.iter().enumerate() {
        if *packet == "[[2]]" || *packet == "[[6]]" {
            key *= (i + 1);
        }
    }

    key
}

pub fn insert_brackets(input: &str, index: usize) -> String {
    let (start, end) = input.split_at(index);
    let next_char = end.as_bytes()[1] as char;
    let (digit, end) = end.split_at(if next_char.is_ascii_digit() { 2 } else { 1 });

    let mut output = start.to_owned();
    output.push('[');
    output.push_str(digit);
    output.push(']');
    output.push_str(end);

    output
}

pub fn compare_p1(left: &str, right: &str, pos: usize) -> std::cmp::Ordering {
    let lch = left.as_bytes()[pos] as char;
    let rch = right.as_bytes()[pos] as char;

    if lch.is_ascii_digit() && rch.is_ascii_digit() {
        let next_left = left.as_bytes()[pos + 1] as char;
        let next_right = right.as_bytes()[pos + 1] as char;

        if next_left.is_ascii_digit() && next_right.is_ascii_digit() {
            // I think 10 is the only 2 digit number, make sure that's safe
            if lch != rch || next_left != next_right {
                println!(
                    "Missing case! pos: {}, {}{} vs {}{}",
                    pos, lch, next_left, rch, next_right
                );
                println!("{}", left);
                println!("{}", right);
                println!(
                    "{}^",
                    std::iter::once(' ').cycle().take(pos).collect::<String>()
                );
                panic!();
            }

            compare_p1(left, right, pos + 2)
        } else if next_left.is_ascii_digit() {
            std::cmp::Ordering::Greater
        } else if next_right.is_ascii_digit() {
            std::cmp::Ordering::Less
        } else if lch == rch {
            compare_p1(left, right, pos + 1)
        } else {
            lch.cmp(&rch)
        }
    } else if lch == rch {
        if pos + 1 == left.len() {
            // assume good inputs, they will be the same length or have a difference before one ends
            std::cmp::Ordering::Equal
        } else {
            compare_p1(left, right, pos + 1)
        }
    } else if lch == '[' && rch.is_ascii_digit() {
        let new_right = insert_brackets(right, pos);

        compare_p1(left, &new_right, pos + 1)
    } else if rch == '[' && lch.is_ascii_digit() {
        let new_left = insert_brackets(left, pos);

        compare_p1(&new_left, right, pos + 1)
    } else if lch == ']' {
        // left is smaller
        std::cmp::Ordering::Less
    } else if rch == ']' {
        // right is smaller
        std::cmp::Ordering::Greater
    } else {
        // Did I forget a case?
        println!("Missing case! pos: {}", pos);
        println!("{}", left);
        println!("{}", right);
        println!(
            "{}^",
            std::iter::once(' ').cycle().take(pos).collect::<String>()
        );

        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator("not_needed").unwrap();
        assert_eq!(solve_part1(&input), 13140);
    }
}
