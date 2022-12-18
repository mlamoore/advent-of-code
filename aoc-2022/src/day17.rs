use aoc_runner_derive::{aoc, aoc_generator};

// true for </left, false for >/right
type Input = Vec<bool>;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Input {
    use aoc_parse::{parser, prelude::*};
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(line({"<" => true, ">" => false}*));

    p.parse(&input).unwrap()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &[bool]) -> usize {
    // input is true for </left, false for >/right
    let mut map = Vec::new();
    let mut rock_type = 0;
    let mut wind_dir = 0;

    for _i in 0..2022 {
        drop_rock(rock_type, &mut map, input, &mut wind_dir);
        rock_type += 1;
        rock_type %= ROCKS.len();
    }

    tower_height(&map)
}

const HISTORY_DEPTH: usize = 10;

#[aoc(day17, part2)]
pub fn solve_part2(input: &[bool]) -> usize {
    // input is true for </left, false for >/right
    let mut map = Vec::new();
    let mut rock_type = 0;
    let mut wind_dir = 0;

    let period = 5 * input.len();

    let target = 1_000_000_000_000;

    let repeats = target / period - 2;

    // Add heights as follows:
    // * Go initial_padding times, measure height
    // * go one complete period, measure height
    // * Calculate final height after 'repeats' more cycles of the period
    let initial_padding = target - (period + period * repeats);

    for _i in 0..initial_padding {
        drop_rock(rock_type, &mut map, input, &mut wind_dir);
        rock_type += 1;
        rock_type %= ROCKS.len();
    }

    let initial_height = tower_height(&map);

    for _i in 0..period {
        drop_rock(rock_type, &mut map, input, &mut wind_dir);
        rock_type += 1;
        rock_type %= ROCKS.len();
    }

    let next_height = tower_height(&map);
    let period_height_gain = next_height - initial_height;

    next_height + repeats * period_height_gain
}

pub fn solve_part2_orig(input: &[bool]) -> usize {
    // input is true for </left, false for >/right
    let mut map = Vec::new();
    let mut rock_type = 0;
    let mut wind_dir = 0;

    let period = 5 * input.len();

    let target = 1_000_000_000_000;

    let repeats = target / period - 2;

    // Add heights as follows:
    // * Go initial_padding times, measure height
    // * go one complete period, measure height
    // * Calculate final height after 'repeats' more cycles of the period
    let initial_padding = target - (period + period * repeats);

    for _i in 0..initial_padding {
        drop_rock(rock_type, &mut map, input, &mut wind_dir);
        rock_type += 1;
        rock_type %= ROCKS.len();
    }

    let initial_height = tower_height(&map);

    for _i in 0..period {
        drop_rock(rock_type, &mut map, input, &mut wind_dir);
        rock_type += 1;
        rock_type %= ROCKS.len();
    }

    let next_height = tower_height(&map);
    let period_height_gain = next_height - initial_height;

    next_height + repeats * period_height_gain
}

// Bit masks of all rock types
// bottom is index 0, left is bit 0
// That is mirrored in x and y relative to problem description
const ROCKS: [&[u8]; 5] = [
    &[0b1111],
    &[0b010, 0b111, 0b010],
    &[0b111, 0b100, 0b100],
    &[0b1, 0b1, 0b1, 0b1],
    &[0b11, 0b11],
];

// (x, y) bounds of all rock types, used for colliding with walls
const ROCK_SIZES: [(usize, usize); 5] = [(4, 1), (3, 3), (3, 3), (1, 4), (2, 2)];

pub fn tower_height(map: &[u8]) -> usize {
    map.iter()
        .enumerate()
        .rev()
        .find(|(_i, row)| **row != 0)
        .map_or(0, |(i, _row)| i + 1)
}

pub fn piece_collides(map: &[u8], rock_type: usize, x: usize, y: usize) -> bool {
    (0..ROCK_SIZES[rock_type].1)
        .into_iter()
        .any(|dy| map[y + dy] & ROCKS[rock_type][dy] << x != 0)
}

pub fn place_piece(map: &mut [u8], rock_type: usize, x: usize, y: usize) {
    for dy in 0..ROCK_SIZES[rock_type].1 {
        map[y + dy] |= ROCKS[rock_type][dy] << x;
    }
}

pub fn pad_map_height(map: &mut Vec<u8>, new_height: usize) {
    for _i in map.len()..new_height {
        map.push(0);
    }
}

// Drops one rock, returns resting place of rock
pub fn drop_rock(
    rock_type: usize,
    map: &mut Vec<u8>,
    wind_patterns: &[bool],
    wind_dir: &mut usize,
) -> (usize, usize) {
    let start_height = tower_height(map);

    pad_map_height(map, start_height + 4 + ROCK_SIZES[rock_type].1);

    let (mut x, mut y) = (2, start_height + 3);

    loop {
        // wind blows
        if wind_patterns[*wind_dir] {
            // blows left
            if x != 0 && !piece_collides(map, rock_type, x - 1, y) {
                x -= 1;
            }
        } else {
            // blows right
            if x != 7 - ROCK_SIZES[rock_type].0 && !piece_collides(map, rock_type, x + 1, y) {
                x += 1;
            };
        }
        *wind_dir += 1;
        *wind_dir %= wind_patterns.len();

        // rock falls
        if y == 0 {
            break;
        } else if piece_collides(map, rock_type, x, y - 1) {
            break;
        } else {
            y -= 1;
        }
    }

    place_piece(map, rock_type, x, y);

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = input_generator(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

        assert_eq!(solve_part2(&input), 1514285714288);
    }
}
