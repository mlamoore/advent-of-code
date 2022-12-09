use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Result<Vec<(char, u32)>, aoc_parse::ParseError> {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(lines(alpha " " u32));

    p.parse(&input)
}

pub fn follow( hx: i32, hy: i32, tx: i32, ty: i32 ) -> (i32, i32) {
    let dx = hx - tx;
    let dy = hy - ty;

    let (dx, dy) = if (dx > 1 && dy > 0) || (dx > 0 && dy > 1) {
        (1, 1)
    }
    else if dx > 1 && dy == 0 {
        (1, 0)
    }
    else if (dx > 1 && dy < 0) || (dx > 0 && dy < -1) {
        (1, -1)
    }
    else if dx == 0 && dy < -1 {
        (0, -1)
    }
    else if (dx < -1 && dy < 0) || (dx < 0 && dy < -1) {
        (-1, -1)
    }
    else if dx < -1 && dy == 0 {
        (-1, 0)
    }
    else if (dx < -1 && dy > 0) || (dx < 0 && dy > 1) {
        (-1, 1)
    }
    else if dx == 0 && dy > 1 {
        (0, 1)
    }
    else {
        (0, 0)
    };

    (tx+dx, ty+dy)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[(char, u32)]) -> usize {

    let mut visited = HashSet::new();

    // head position
    let mut hx = 0;
    let mut hy = 0;

    // tail position
    let mut tx = 0;
    let mut ty = 0;

    visited.insert((tx, ty));

    for (dir, count) in input.into_iter() {
        //println!("== {} {} ==", dir, count);

        for _ in 0..*count {
            match dir {
                'U' => hy += 1,
                'D' => hy -= 1,
                'L' => hx -= 1,
                'R' => hx += 1,
                _ => panic!()
            };

            (tx, ty) = follow( hx, hy, tx, ty );

            visited.insert((tx, ty));

            //debug_visualize( &visited, hx, hy, tx, ty );
        }
    }

    //debug_visualize( &visited, 0, 0, 0, 0 );

    visited.iter().count()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[(char, u32)]) -> usize {

    let mut visited = HashSet::new();

    // rope position (0 = head, 9 = last tail)
    let mut rx = [0; 10];
    let mut ry = [0; 10];

    visited.insert((rx[9], ry[9]));

    for (dir, count) in input.into_iter() {

        for _ in 0..*count {
            match dir {
                'U' => ry[0] += 1,
                'D' => ry[0] -= 1,
                'L' => rx[0] -= 1,
                'R' => rx[0] += 1,
                _ => panic!()
            };

            for i in 1..10 {
                for _ in 0..10 {
                    // Apply follow repeatedly to be safe, in case of big motions
                    (rx[i], ry[i]) = follow( rx[i-1], ry[i-1], rx[i], ry[i]);
                }
            }

            visited.insert((rx[9], ry[9]));

            //debug_visualize( &visited, hx, hy, tx, ty );
        }
    }

    //debug_visualize( &visited, 0, 0, 0, 0 );

    visited.iter().count()
}

fn debug_visualize( visited: &HashSet<(i32, i32)>, hx: i32, hy: i32, tx: i32, ty: i32 ) {
    let xmin = 0;
    let xmax = 5;
    let ymin = 0;
    let ymax = 4;

    println!("");

    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            if hx == x && hy == y {
                print!("H");
            }
            else if tx == x && ty == y {
                print!("T");
            }
            else if x == 0 && y == 0 {
                print!("s");
            }
            else if visited.contains(&(x, y)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }

    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        )
        .unwrap();
        assert_eq!(solve_part1(&input), 13);
    }
}
