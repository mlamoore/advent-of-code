use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<Vec<u8>>, (usize, usize), (usize, usize));

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(lines(alpha*));

    let alpha_grid = p.parse(&input).unwrap();

    let mut map = vec![vec![0; alpha_grid[0].len()]; alpha_grid.len()];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..alpha_grid.len() {
        for x in 0..alpha_grid[0].len() {
            let sq = alpha_grid[y][x];

            map[y][x] = if sq.is_ascii_lowercase() {
                sq as u8 - 'a' as u8
            } else if sq == 'S' {
                start = (x, y);
                0
            } else if sq == 'E' {
                end = (x, y);
                25
            } else {
                panic!()
            };
        }
    }

    (map, start, end)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let map = &input.0;
    let start = input.1;
    let end = input.2;

    find_shortest(map, start, end, i64::MAX)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    let map = &input.0;
    let orig_start = input.1;
    let end = input.2;
    let height = map.len();
    let width = map[0].len();

    let mut shortest = find_shortest(map, orig_start, end, i64::MAX);

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 0 {
                shortest = shortest.min(find_shortest(map, (x, y), end, shortest));
            }
        }
    }

    shortest
}

pub fn find_shortest(
    map: &[Vec<u8>],
    start: (usize, usize),
    end: (usize, usize),
    max_allowed: i64,
) -> i64 {
    let start = (start.0 as i32, start.1 as i32);
    let end = (end.0 as i32, end.1 as i32);
    let height = map.len();
    let width = map[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut tent_distance = vec![vec![i64::MAX; width]; height];
    tent_distance[start.1 as usize][start.0 as usize] = 0;

    loop {
        let (nx, ny) = find_best(&visited, &tent_distance);

        if tent_distance[ny as usize][nx as usize] >= max_allowed {
            // The best possible path is too long
            return i64::MAX;
        }

        for delta in [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter() {
            let (px, py) = (nx + delta.0, ny + delta.1);
            if can_visit(px, py, map[ny as usize][nx as usize], map, &visited) {
                tent_distance[py as usize][px as usize] = tent_distance[py as usize][px as usize]
                    .min(tent_distance[ny as usize][nx as usize] + 1);
            }
        }

        visited[ny as usize][nx as usize] = true;

        if nx == end.0 && ny == end.1 {
            return tent_distance[ny as usize][nx as usize];
        }
    }
}

fn can_visit(x: i32, y: i32, old_height: u8, map: &[Vec<u8>], visited: &[Vec<bool>]) -> bool {
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        false
    } else if visited[y as usize][x as usize] {
        false
    } else {
        old_height + 1 >= map[y as usize][x as usize]
    }
}

// TODO the right sort of MinHeap/HashMap combo would change this from O(n^2) to O(1), but you would need to find by one key and sort by a different key (or keep two different versions)
fn find_best(visited: &[Vec<bool>], tent_distance: &[Vec<i64>]) -> (i32, i32) {
    let (mut bx, mut by) = (0, 0);
    let mut bcost = i64::MAX;

    for x in 0..visited[0].len() {
        for y in 0..visited.len() {
            if !visited[y][x] && tent_distance[y][x] < bcost {
                bcost = tent_distance[y][x];
                bx = x;
                by = y;
            }
        }
    }

    (bx as i32, by as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator("not_needed");

        assert_eq!(solve_part1(&input), 0);
    }
}
