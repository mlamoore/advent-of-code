use aoc_runner_derive::{aoc, aoc_generator};


// Is there a sea cucumber there, and if so is it facing east?
#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<Vec<Option<bool>>> {
    input.lines().map(|l| l.trim().chars().map(|t| {
        match t {
            '.' => None,
            '>' => Some(true),
            'v' => Some(false),
            _ => panic!(),
        }
    }).collect()).collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(state: &[Vec<Option<bool>>]) -> usize {
    let mut state = state.to_vec();

    let mut steps = 0;

    loop {
        let (new_state, changed) = advance(&state);

        steps += 1;

        if !changed {
            break steps
        }

        state = new_state;
    }
}


pub fn advance(state: &[Vec<Option<bool>>]) -> (Vec<Vec<Option<bool>>>, bool) {
    let width = state[0].len();
    let height = state.len();

    let mut changed = false;
    let mut next = state.to_vec();

    // Pass for east facing 'true' cucumbers
    for y in 0..height {
        for x in 0..width {
            // Check if next spot is open
            if state[y][x] == Some(true) && state[y][(x + 1) % width].is_none() {
                next[y][x] = None;
                next[y][(x + 1) % width] = Some(true);
                changed = true;
            }
        }
    }

    let state = next;
    let mut next = state.clone();

    // Pass for south facing 'false' cucumbers
    for y in 0..height {
        for x in 0..width {
            // Check if next spot is open
            if state[y][x] == Some(false) && state[(y + 1) % height][x].is_none() {
                next[y][x] = None;
                next[(y + 1) % height][x] = Some(false);
                changed = true;
            }
        }
    }

    (next, changed)
}

pub fn display(state: &[Vec<Option<bool>>]) {
    for row in state {
        println!("{}", row.iter().map(|px| match px {
            None => '.',
            Some(dir) => if *dir { '>' } else { 'v' },
        }).collect::<String>());
    }
}

#[test]
pub fn test_ex1() {
    let input = "v...>>.vv>\n.vv>>.vv..\n>>.>v>...v\n>>v>>.>.v.\nv>v.vv.v..\n>.>>..v...\n.vv..>.>v.\nv.v..>>v.v\n....v..v.>\n";

    let mut map = input_generator(input);

    println!("Initial state:");
    display(&map);

    for steps in 0..5 {
        let (next, changed) = advance( &map);

        map = next;

        println!("State after {} steps, changed = {}", steps, changed);
        display(&map);
    }
}


#[test]
pub fn test_ex1_full() {
    let input = "v...>>.vv>\n.vv>>.vv..\n>>.>v>...v\n>>v>>.>.v.\nv>v.vv.v..\n>.>>..v...\n.vv..>.>v.\nv.v..>>v.v\n....v..v.>\n";

    let map = input_generator(input);

    let steps = solve_part1( &map);

    println!("Example in {} steps", steps);
}

#[test]
pub fn test_basic() {
    let input = "...>...\n.......\n......>\nv.....>\n......>\n.......\n..vvv..\n";

    let mut map = input_generator(input);

    println!("Initial state:");
    display(&map);

    for steps in 0..5 {
        let (next, changed) = advance( &map);

        map = next;

        println!("State after {} steps, changed = {}", steps, changed);
        display(&map);
    }
}
