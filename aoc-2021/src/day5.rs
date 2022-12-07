use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<(usize, usize, usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.trim().split(" -> ");
            let mut p1 = it.next().unwrap().split(',');
            let mut p2 = it.next().unwrap().split(',');
            (
                p1.next().unwrap().parse().unwrap(),
                p1.next().unwrap().parse().unwrap(),
                p2.next().unwrap().parse().unwrap(),
                p2.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn visualize(board: &[usize], stride: usize) {
    let height = board.len() / stride;

    for y in 0..height {
        let row = &board[y * stride..(y + 1) * stride];
        let row = row
            .iter()
            .map(|i| {
                if *i == 0 {
                    '.'
                } else if *i >= 15 {
                    'F'
                } else {
                    char::from_digit(*i as u32, 16).unwrap()
                }
            })
            .collect::<String>();

        println!("{}", row);
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[(usize, usize, usize, usize)]) -> usize {
    let xmax = input
        .iter()
        .map(|(x1, x2, _y1, _y2)| if x1 > x2 { x1 } else { x2 })
        .fold(0, |prev, new| if prev > *new { prev } else { *new });
    let ymax = input
        .iter()
        .map(|(_x1, _x2, y1, y2)| if y1 > y2 { y1 } else { y2 })
        .fold(0, |prev, new| if prev > *new { prev } else { *new });

    let mut hits = vec![0usize; (xmax + 1) * (ymax + 1)];

    //println!( "Input: " );
    //println!( "{:#?}", input );

    for (x1, y1, x2, y2) in input {
        if x1 == x2 {
            let (ymn, ymx) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            //println!( "({}, {}) -> ({}, {})", x1, y1, x2, y2 );
            for y in *ymn..(*ymx + 1) {
                hits[x1 + xmax * y] += 1;
            }
        } else if y1 == y2 {
            let (xmn, xmx) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            //println!( "({}, {}) -> ({}, {})", x1, y1, x2, y2 );
            for x in *xmn..(*xmx + 1) {
                hits[x + xmax * y1] += 1;
            }
        } else {
            // Diagonal
        }
    }

    //println!( "Result: " );
    //visualize( &hits, xmax );

    hits.iter().filter(|n| **n > 1).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[(usize, usize, usize, usize)]) -> usize {
    let xmax = input
        .iter()
        .map(|(x1, x2, _y1, _y2)| if x1 > x2 { x1 } else { x2 })
        .fold(0, |prev, new| if prev > *new { prev } else { *new });
    let ymax = input
        .iter()
        .map(|(_x1, _x2, y1, y2)| if y1 > y2 { y1 } else { y2 })
        .fold(0, |prev, new| if prev > *new { prev } else { *new });

    let mut hits = vec![0usize; (xmax + 1) * (ymax + 1)];

    //println!( "Input: " );
    //println!( "{:#?}", input );

    for (x1, y1, x2, y2) in input {
        if x1 == x2 {
            let (ymn, ymx) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            //println!( "({}, {}) -> ({}, {})", x1, y1, x2, y2 );
            for y in *ymn..(*ymx + 1) {
                hits[x1 + xmax * y] += 1;
            }
        } else if y1 == y2 {
            let (xmn, xmx) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            //println!( "({}, {}) -> ({}, {})", x1, y1, x2, y2 );
            for x in *xmn..(*xmx + 1) {
                hits[x + xmax * y1] += 1;
            }
        } else {
            // (xmn, y1) to (xmx, y2)
            let (xmn, xmx, y1, y2) = if x1 < x2 {
                (x1, x2, y1, y2)
            } else {
                (x2, x1, y2, y1)
            };

            for x in *xmn..(*xmx + 1) {
                let y = if y1 < y2 { y1 + x - xmn } else { y1 + xmn - x };
                hits[x + xmax * y] += 1;
            }
        }
    }

    //println!( "Result: " );
    //visualize( &hits, xmax );

    hits.iter().filter(|n| **n > 1).count()
}
