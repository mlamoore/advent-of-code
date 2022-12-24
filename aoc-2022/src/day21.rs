use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Monkey {
    Calc(String, Op, String),
    Num(i64),
    Linear { num: i64, denom: i64, b: i64 }, // (x * num + b) / denom
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

impl Monkey {
    fn eval(&self, all: &HashMap<String, Monkey>) -> Monkey {
        match self {
            Monkey::Num(x) => Monkey::Num(*x),
            Monkey::Linear { num, denom, b } => Monkey::Linear { num: *num, denom: *denom, b: *b },
            Monkey::Calc(m1, op, m2) => {
                let m1 = all[m1].eval(all);
                let m2 = all[m2].eval(all);
                match (m1, m2) {
                    (Monkey::Num(a), Monkey::Num(b)) => Monkey::Num(op.eval(a, b)),
                    (Monkey::Linear { num, denom, b }, Monkey::Num(modifier)) => match op {
                        Op::Mul => Monkey::Linear { num: num * modifier, denom, b: b * modifier },
                        Op::Div => {
                            let gcd = gcd(gcd(num, modifier), b);
                            Monkey::Linear { num: num / gcd, denom: denom * modifier / gcd, b: b / gcd }
                        },
                        Op::Add => Monkey::Linear { num, denom, b: b + modifier * denom },
                        Op::Sub => Monkey::Linear { num, denom, b: b - modifier * denom },
                    },
                    (Monkey::Num(modifier), Monkey::Linear { num, denom, b }) => match op {
                        Op::Mul => Monkey::Linear { num: num * modifier, denom, b: b * modifier },
                        Op::Div => panic!(),
                        Op::Add => Monkey::Linear { num, denom, b: b + modifier * denom },
                        Op::Sub => Monkey::Linear { num: -num, denom, b: modifier * denom - b },
                    },
                    _ => panic!(),
                }
            },
        }
    }
}

impl Op {
    fn eval(&self, in1: i64, in2: i64) -> i64 {
        match self {
            Op::Mul => in1 * in2,
            Op::Div => in1 / in2,
            Op::Add => in1 + in2,
            Op::Sub => in1 - in2,
        }
    }
}

pub fn gcd(in1: i64, in2: i64) -> i64 {
    if in2 == 0 {
        in1
    }
    else {
        gcd(in2, in1 % in2)
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> HashMap<String, Monkey> {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let name = parser!(string(alpha+));
    let operator = parser!({
        '*' => Op::Mul,
        '/' => Op::Div,
        '+' => Op::Add,
        '-' => Op::Sub,
    });

    //let p = parser!(lines({"noop" => CpuInst::NoOp, "addx " (x: i32) => CpuInst::AddX(x)}));
    let p = parser!(lines(name ": " {
        x:i64 => Monkey::Num(x),
        monkey1:name ' ' op:operator ' ' monkey2:name => Monkey::Calc(monkey1, op, monkey2),
    }));

    let v = p.parse(&input).unwrap();

    let mut output = HashMap::new();

    for (name, monkey) in v.into_iter() {
        output.insert(name, monkey);
    }

    output
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &HashMap<String, Monkey>) -> i64 {
    if let Monkey::Num(x) = input["root"].eval(input) {
        x
    }
    else {
        panic!();
    }
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &HashMap<String, Monkey>) -> i64 {
    let mut input = input.clone();

    input.insert("humn".to_owned(), Monkey::Linear { num: 1, denom: 1, b: 0 } );
    let input = input; // no longer mutable

    match &input["root"] {
        Monkey::Calc(m1, Op::Add, m2) => {
            let m1 = input[m1].eval(&input);
            let m2 = input[m2].eval(&input);

            println!("Comparing {:?} to {:?}", m1, m2);

            let (num, denom, b, other) = match (m1, m2) {
                (Monkey::Linear { num, denom, b }, Monkey::Num(other)) => (num, denom, b, other),
                (Monkey::Num(other), Monkey::Linear { num, denom, b }) => (num, denom, b, other),
                _ => panic!(),
            };

            // x * num = denom * other - b
            (denom * other - b) / num
        },
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(35,25), 5);
        assert_eq!(gcd(35,-25), -5);
        assert_eq!(gcd(-35,25), 5);
        assert_eq!(gcd(-35,-25), -5);
        assert_eq!(gcd(179159040,-2977118878569840640), 81920);
    }
}
