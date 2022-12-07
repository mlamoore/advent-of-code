use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::hash_set::HashSet;
//use crate::util;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.trim().chars().map(|t| t.to_digit(10).unwrap() as u8).collect()).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Vec<u8>]) -> usize {
    let start = (0, 0);
    let end = (input[0].len()-1, input.len()-1);

    a_star( start, end, input ).0
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Vec<u8>]) -> usize {
    let width = input[0].len();
    let height = input.len();

    let mut map = vec![vec![0; 5 * width]; 5 * height];

    for ytile in 0..5 {
        for y in 0..height {
            for xtile in 0..5 {
                for x in 0..height {
                    map[ytile * height + y][xtile * width + x] = ((input[y][x] + xtile as u8 + ytile as u8) - 1) % 9 + 1;
                }
            }
        }
    }


    let start = (0, 0);
    let end = (map[0].len()-1, map.len()-1);

    a_star( start, end, &map ).0
}


#[test]
pub fn test_part1() {
    let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n";

    let map = input_generator( input );

    let start = (0, 0);
    let end = (map[0].len()-1, map.len()-1);

    


    println!( "Final solution: {:?}", a_star( start, end, &map ) );
}

pub fn hueristic( node: (usize, usize), end: (usize, usize) ) -> usize {
    (isize::abs(node.0 as isize - end.0 as isize) + isize::abs(node.1 as isize - end.1 as isize)) as usize
}

pub fn a_star( start: (usize, usize), end: (usize, usize), map: &[Vec<u8>]) -> (usize, Vec<(usize, usize)>) {
    let mut open_set: HashSet<(usize, usize)> = HashSet::new();

    open_set.insert( start );

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();

    g_score.insert( start, 0 );

    let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();

    f_score.insert( start, hueristic( start, end ) );

    while !open_set.is_empty() {
        //println!( "----------------" );
        //println!( "Open set: {:?}", open_set );
        //println!( "g_score: {:?}", g_score );
        //println!( "f_score: {:?}", f_score );
        let first_open = *open_set.iter().next().unwrap();
        let current = open_set.iter().fold(first_open, |best, next| if f_score.get(&best).unwrap() < f_score.get(next).unwrap() { best } else { *next } );

        //println!( "Now considering {:?}", current );

        if current == end {
            //println!( "Found the end!!!" );
            let mut path = Vec::new();

            let mut current = end;

            while current != start {
                path.push(current);
                current = *came_from.get(&current).unwrap();
            }

            path.push(current);

            return (*g_score.get(&end).unwrap(), path.iter().rev().cloned().collect());
        }

        open_set.remove(&current);

        let mut neighbors = Vec::new();

        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < end.0 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < end.1 {
            neighbors.push((current.0, current.1 + 1));
        }

        for neighbor in neighbors {
            let tent_gscore = *g_score.get(&current).unwrap() + map[neighbor.1][neighbor.0] as usize;

            let ngs = g_score.get(&neighbor);

            //println!( "    Considering {:?}", neighbor );
            //println!( "    This path score: {:?}, existing path: {:?}", tent_gscore, ngs );

            if tent_gscore < *ngs.unwrap_or(&usize::MAX) {
                //println!( "    FOUND NEW BEST" );
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tent_gscore);
                f_score.insert(neighbor, tent_gscore + hueristic(neighbor, end));
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }

        }
    }

    panic!();
}


pub fn a_star_array( start: (usize, usize), end: (usize, usize), map: &[Vec<u8>]) -> (usize, Vec<(usize, usize)>) {
    let width = map[0].len();
    let height = map.len();

    let mut open_set: HashSet<(usize, usize)> = HashSet::new();

    open_set.insert( start );

    let mut g_score = vec![usize::MAX; width * height];
    let mut f_score = vec![usize::MAX; width * height];
    f_score[start.0 + width * start.1] = hueristic(start, end);

    while !open_set.is_empty() {
        //println!( "----------------" );
        //println!( "Open set: {:?}", open_set );
        //println!( "g_score: {:?}", g_score );
        //println!( "f_score: {:?}", f_score );
        let first_open = *open_set.iter().next().unwrap();
        let current = open_set.iter().fold(first_open, |best, next| if f_score[best.0 + width * best.1] < f_score[next.0 + width * next.1] { best } else { *next } );

        //println!( "Now considering {:?}", current );

        if current == end {
            //println!( "Found the end!!!" );
            return (g_score[end.0 + width * end.1], Vec::new());
        }

        open_set.remove(&current);

        let mut neighbors = Vec::new();

        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < end.0 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < end.1 {
            neighbors.push((current.0, current.1 + 1));
        }

        for neighbor in neighbors {
            let tent_gscore = g_score[current.0 + width * current.1] + map[neighbor.1][neighbor.0] as usize;

            let ngs = g_score[neighbor.0 + width * current.1];

            if tent_gscore < ngs {
                g_score[neighbor.0 + width * neighbor.1] = tent_gscore;
                f_score[neighbor.0 + width * neighbor.1] = tent_gscore + hueristic(neighbor, end);
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }

        }
    }

    panic!();
}
