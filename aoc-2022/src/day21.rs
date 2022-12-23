use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Monkey {
    Calc(String, Op, String),
    Num(i64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

impl Monkey {
    fn eval(&self, all: &HashMap<String, Monkey>) -> i64 {
        match self {
            Monkey::Calc(m1, op, m2) => op.eval(all[m1].eval(all), all[m2].eval(all)),
            Monkey::Num(x) => *x,
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
    input["root"].eval(input)
}
