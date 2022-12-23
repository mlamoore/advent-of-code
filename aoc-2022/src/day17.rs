use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

#[aoc(day17, part2)]
pub fn solve_part2(input: &[bool]) -> usize {
    let mut transitions: HashMap<(Vec<u8>, usize), (Vec<u8>, usize)> = HashMap::new();

    let mut remaining = 1_000_000_000_000;
    let mut total_height = 0;

    let mut state = drop_chunk_rocks(&[], input, &mut remaining, &mut total_height);

    display_chunk(&state, "first chunk");

    while remaining > 0 {
        if transitions.contains_key(&state) {
            // We found a cycle!
            // Starting at state, get back to state

            println!("Found a cycle!!!");
            display_chunk(&state, "cycle start");

            let period = ROCKS.len() * input.len();
            let mut cycle_length = 1;
            let mut height_gained = state.1; // This would be the last, but use it as the first

            let start_state = &state;

            let mut next_state: &(Vec<u8>, usize) = transitions.get(&state).unwrap();

            while next_state != start_state {
                cycle_length += 1;
                height_gained += next_state.1;
                next_state = transitions.get(next_state).unwrap();
            }

            let num_cycles = remaining / (cycle_length * period);

            remaining -= num_cycles * cycle_length * period;
            total_height += num_cycles * height_gained;

            let mut remembered = &state;

            while remaining > period {
                remembered = transitions.get(remembered).unwrap();
                total_height += remembered.1;
                remaining -= period;
            }

            // Less than one period remaining, calculate the rest
            drop_chunk_rocks(&state.0, input, &mut remaining, &mut total_height);
        } else {
            let new_state = drop_chunk_rocks(&state.0, input, &mut remaining, &mut total_height);

            transitions.insert(state, new_state.clone());

            state = new_state;

            display_chunk(&state, "newly explored");
        }
    }

    total_height
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn display_chunk(state: &(Vec<u8>, usize), title: &str) {
    println!(
        "Chunk ({}) additional height = {}, hash = {}",
        title,
        state.1,
        calculate_hash(state)
    );

    for bit in 0..7 {
        println!(
            "    {}",
            state
                .0
                .iter()
                .rev()
                .map(|row| if row & (1 << bit) == 0 { '.' } else { '#' })
                .collect::<String>()
        );
    }

    println!("");
}

// rock_type and wind_dir must be 0 at the start of each chunk
// they will be 0 at the end of the chunk unless remaining reaches 0
pub fn drop_chunk_rocks(
    state: &[u8],
    wind_patterns: &[bool],
    remaining: &mut usize,
    total_height: &mut usize,
) -> (Vec<u8>, usize) {
    let mut map = state.to_vec();
    let mut rock_type = 0;
    let mut wind_dir = 0;

    let period = ROCKS.len() * wind_patterns.len();

    let num_rocks = period.min(*remaining);

    let start_height = tower_height(&map);

    for _i in 0..num_rocks {
        drop_rock(rock_type, &mut map, wind_patterns, &mut wind_dir);
        rock_type += 1;
        rock_type %= ROCKS.len();
    }

    *remaining -= num_rocks;

    let height = tower_height(&map);
    let height_gained = height - start_height;
    *total_height += height_gained;

    // If this wasn't an empty start, make sure the bottom row didn't change, otherwise we need deeper history
    if state.len() > 0 {
        assert_eq!(state[0], map[0]);
    }

    (get_state(&map), height_gained)
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

pub fn get_state(map: &[u8]) -> Vec<u8> {
    const HISTORY_DEPTH: usize = 50;

    map[(map.len() - map.len().min(HISTORY_DEPTH))..map.len()].to_vec()
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

    pad_map_height(map, start_height + 3 + ROCK_SIZES[rock_type].1);

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

        //assert_eq!(solve_part2(&input), 1514285714288);
    }
}
