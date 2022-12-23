use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> HashSet<(i64, i64)> {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(lines({"#" => true, "." => false}*));

    let map = p.parse(&input).unwrap();

    let mut output = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] {
                output.insert((x as i64, y as i64));
            }
        }
    }

    output
}

const NEIGHBORS: [[(i64, i64); 3]; 4] = [
    // NSWE offsets
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
];

#[aoc(day23, part1)]
pub fn solve_part1(input: &HashSet<(i64, i64)>) -> usize {
    let mut elves = input.clone();
    let mut first_dir = 0;

    //println!("Initial state" );
    //debug_visualize( &elves, -3, 11, -3, 11 );

    for round in 0..10 {
        elves = move_all(&elves, first_dir);
        first_dir += 1;
        first_dir %= 4;

        //println!("After round {}", round + 1 );
        //debug_visualize( &elves, -3, 11, -3, 11 );
    }

    let xmin = elves.iter().map(|(x, _y)| x).min().unwrap();
    let xmax = elves.iter().map(|(x, _y)| x).max().unwrap();
    let ymin = elves.iter().map(|(_x, y)| y).min().unwrap();
    let ymax = elves.iter().map(|(_x, y)| y).max().unwrap();

    (xmax - xmin + 1) as usize * (ymax - ymin + 1) as usize - elves.len()
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &HashSet<(i64, i64)>) -> usize {
    let mut elves = input.clone();
    let mut round = 0;

    loop {
        let new_elves = move_all(&elves, round % 4);
        round += 1;

        if new_elves == elves {
            break round;
        }

        elves = new_elves;
    }
}

pub fn move_all(elves: &HashSet<(i64, i64)>, first_dir: usize) -> HashSet<(i64, i64)> {
    let mut proposed = HashMap::new();

    for &(x, y) in elves {
        let (mut px, mut py) = (x, y);

        if NEIGHBORS
            .iter()
            .flat_map(|dir| dir.iter())
            .any(|&(dx, dy)| elves.contains(&(x + dx, y + dy)))
        {
            for dir in 0..4 {
                if NEIGHBORS[(dir + first_dir) % 4]
                    .iter()
                    .all(|&(dx, dy)| !elves.contains(&(x + dx, y + dy)))
                {
                    let (dx, dy) = NEIGHBORS[(dir + first_dir) % 4][1];
                    px = x + dx;
                    py = y + dy;
                    break;
                }
            }
        }

        proposed.insert((x, y), (px, py));
    }

    let mut new_pos = HashSet::new();

    for ((x, y), (px, py)) in proposed.iter() {
        if proposed
            .iter()
            .all(|((ox, oy), (opx, opy))| px != opx || py != opy || (x == ox && y == oy))
        {
            // valid move
            new_pos.insert((*px, *py));
        } else {
            new_pos.insert((*x, *y));
        }
    }

    new_pos
}

pub fn debug_visualize(visited: &HashSet<(i64, i64)>, xmin: i64, xmax: i64, ymin: i64, ymax: i64) {
    println!("");

    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            if visited.contains(&(x, y)) {
                print!("#");
            } else {
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
            ".....
..##.
.#...
....#
.....
..#..",
        );

        assert_eq!(solve_part1(&input), 110);
    }
}
