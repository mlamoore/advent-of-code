use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(i32, i32, i32, i32)>;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    let mut input = input.to_owned();
    input.push('\n');

    let p =
        parser!(lines("Sensor at x=" i32 ", y=" i32 ": closest beacon is at x=" i32 ", y=" i32));

    p.parse(&input).unwrap()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[(i32, i32, i32, i32)]) -> usize {
    let min_x = input
        .iter()
        .map(|(sx, _sy, bx, _by)| sx.min(bx))
        .min()
        .unwrap();
    let max_x = input
        .iter()
        .map(|(sx, _sy, bx, _by)| sx.max(bx))
        .max()
        .unwrap();
    let max_d = input
        .iter()
        .map(|(sx, sy, bx, by)| (sx - bx).abs() + (sy - by).abs())
        .max()
        .unwrap();

    let cx = -min_x + 2 * max_d;

    let mut row = vec![false; (max_x - min_x + 4 * max_d + 1) as usize];

    let y = 2_000_000;

    for &(sx, sy, bx, by) in input.iter() {
        let d = (sx - bx).abs() + (sy - by).abs();
        let dy = (sy - y).abs();

        if dy <= d {
            let dx = d - dy;
            for x in (sx - dx)..=(sx + dx) {
                row[(x + cx) as usize] = true;
            }
        }
    }

    for &(_sx, _sy, bx, by) in input.iter() {
        if by == y {
            row[(bx + cx) as usize] = false;
        }
    }

    row.iter().filter(|x| **x).count()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[(i32, i32, i32, i32)]) -> i32 {
    big_search(input, 4_000_000)
}

pub fn big_search(input: &[(i32, i32, i32, i32)], limit: i32) -> i32 {
    for y in 0..=limit {
        let mut x = 0;

        'search_x: while x <= limit {
            for &(sx, sy, bx, by) in input.iter() {
                let db = (sx - bx).abs() + (sy - by).abs();
                let dh = (sx - x).abs() + (sy - y).abs();

                if dh <= db {
                    let dby = (sy - y).abs();
                    let next_x = sx + db - dby + 1;

                    //println!("At ({}, {}), sensor ({}, {}) has distance {}, we're closer at {}; y distance is {} so moving forward {} is inside so we want x={}", x, y, sx, sy, db, dh, dby, db-dby, next_x);

                    x = next_x;

                    continue 'search_x;
                }
            }

            println!("Found spot at ({}, {})", x, y);
            for &(sx, sy, bx, by) in input.iter() {
                let db = (sx - bx).abs() + (sy - by).abs();
                let dh = (sx - x).abs() + (sy - y).abs();

                println!(
                    "    Sensor ({}, {}) has distance {}, we're {} at {}",
                    sx,
                    sy,
                    db,
                    if dh < db {
                        "CLOSER"
                    } else if dh == db {
                        "EQUAL"
                    } else {
                        "farther"
                    },
                    dh
                );
            }

            // If we didn't hit a continue above, we found it!
            return x * 4_000_000 + y;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = input_generator(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        );

        assert_eq!(big_search(&input, 20), 56000011);
    }
}
