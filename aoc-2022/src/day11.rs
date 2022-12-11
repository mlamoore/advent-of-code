use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monkey {
    index: usize,
    starting_items: Vec<usize>,
    inspection: Operation,
    test: usize,
    next_true: usize,
    next_false: usize,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    let mut input = input.to_owned();
    input.push('\n');

    let op = parser!({
        "* old" => Operation::Square,
        "* " (x: usize) => Operation::Multiply(x),
        "+ " (x: usize) => Operation::Add(x),});

    let p = parser!(sections(
        (index: line("Monkey " usize ":"))
        (starting_items: line("  Starting items: " repeat_sep(usize, ", ")))
        (inspection: line("  Operation: new = old " op))
        (test: line("  Test: divisible by " usize))
        (next_true: line("    If true: throw to monkey " usize))
        (next_false: line("    If false: throw to monkey " usize))
        => Monkey { index, starting_items, inspection, test, next_true, next_false, }));

    p.parse(&input).unwrap()
}

pub fn execute_round(
    monkies: &[Monkey],
    items: &mut [Vec<usize>],
    activity: &mut [usize],
    worry_divisor: usize,
) {
    let num = monkies.len().min(items.len()).min(activity.len());
    let worry_modulo: usize = monkies.iter().map(|monkey| monkey.test).product();

    for monkey_i in 0..num {
        let monkey = &monkies[monkey_i];
        let num_items = items[monkey_i].len();
        activity[monkey_i] += num_items;
        for item_i in 0..num_items {
            let item = items[monkey_i][item_i];
            let item = match monkey.inspection {
                Operation::Add(x) => item + x,
                Operation::Multiply(x) => item * x,
                Operation::Square => item * item,
            };
            let item = item / worry_divisor;
            let item = item % worry_modulo;
            let next_monkey = if item % monkey.test == 0 {
                monkey.next_true
            } else {
                monkey.next_false
            };
            items[next_monkey].push(item);
        }
        items[monkey_i].clear();
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> usize {
    let num_monkies = input.len();
    let mut items = input
        .iter()
        .map(|monkey| monkey.starting_items.clone())
        .collect::<Vec<_>>();
    let mut activity = vec![0; num_monkies];

    for _round in 0..20 {
        execute_round(input, &mut items, &mut activity, 3);
    }

    activity.sort();

    activity[num_monkies - 1] * activity[num_monkies - 2]
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Monkey]) -> usize {
    let num_monkies = input.len();
    let mut items = input
        .iter()
        .map(|monkey| monkey.starting_items.clone())
        .collect::<Vec<_>>();
    let mut activity = vec![0; num_monkies];

    for _round in 0..10_000 {
        execute_round(input, &mut items, &mut activity, 1);
    }

    activity.sort();

    activity[num_monkies - 1] * activity[num_monkies - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1\n",
        );

        assert_eq!(solve_part1(&input), 10605);
    }
}
