use aoc_runner_derive::aoc;

#[aoc(day23, part1)]
pub fn solve_part1(_input: &str) -> usize {
    // Solved by hand in day23-solve1.txt
    3+3+50+2+30+400+600+6000+6000+300+60+7
}

#[aoc(day23, part2)]
pub fn solve_part2(_input: &str) -> usize {
    // state.0 = 7 top positions
    // state.1 = 4 rooms of 4 slots
    // state.1[1][0] is B column, top slot
    let state: ([Option<u8>; 7], [[Option<u8>; 4]; 4]) = ([None; 7], [[Some(0), Some(3), Some(3), Some(1)], [Some(3), Some(2), Some(1), Some(2)], [Some(0), Some(1), Some(0), Some(3)], [Some(1), Some(0), Some(2), Some(2)]]);

    find_min_path(&state, 0)
}

// Checks room avialability, returns Some(slot) if the room has bottommost available slot [slot]
pub fn room_available(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize) -> Option<usize> {
    for i in 0..4 {
        let check = 3 - i;

        match state.1[room][check] {
            None => return Some(check),
            Some(occupied) => { if occupied as usize != check { return None; } },
        }
    }

    None
}

// Checks leftmost available slot index
pub fn mid_available_left(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize) -> Option<usize> {

    // Check if immediate right spot is available, move left
    let mut left_spot = room + 1;

    if !state.0[left_spot].is_none() {
        if state.0[left_spot + 1].is_none() {
            Some(left_spot + 1)
        }
        else {
            None
        }
    }
    else {
        while left_spot > 0 {
            if state.0[left_spot-1].is_some() {
                return Some(left_spot);
            }
            left_spot -= 1;
        }
        Some(0)
    }
}

// Checks rightmost available slot index
pub fn mid_available_right(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize) -> Option<usize> {

    // Check if immediate left spot is available, move right
    let mut right_spot = room + 2;

    if !state.0[right_spot].is_none() {
        if state.0[right_spot - 1].is_none() {
            Some(right_spot - 1)
        }
        else {
            None
        }
    }
    else {
        while right_spot < 6 {
            if state.0[right_spot+1].is_some() {
                return Some(right_spot);
            }
            right_spot += 1;
        }
        Some(6)
    }
}

// Checks left full slot index
pub fn first_full_left(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize) -> Option<usize> {

    // Check if immediate right spot is available, move left
    let mut left_spot = room + 1;

    while left_spot > 0 {
        if state.0[left_spot].is_some() {
            return Some(left_spot);
        }
        left_spot -= 1;
    }

    if state.0[0].is_some() {
        Some(0)
    }
    else {
        None
    }
}

// Checks right full slot index
pub fn first_full_right(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize) -> Option<usize> {

    // Check if immediate right spot is available, move left
    let mut right_spot = room + 2;

    while right_spot < 7 {
        if state.0[right_spot].is_some() {
            return Some(right_spot);
        }
        right_spot += 1;
    }

    None
}

pub fn slot_path_to_room(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize, slot: usize) -> bool {
    let right_slot = room + 2;

    if slot >= right_slot {
        // Need to move to left
        for i in right_slot..slot {
            if state.0[i].is_some() {
                return false;
            }
        }
        true
    }
    else {
        // Need to move to right
        for i in slot+1..right_slot {
            if state.0[i].is_some() {
                return false;
            }
        }
        true
    }
}

pub fn top_escape_room_depth(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize) -> Option<usize> {
    let mut dirty = false;

    for i in 0..4 {
        let depth = 3 - i;
        let occ = state.1[room][depth];

        if occ.is_none() {
            if dirty && depth < 3 {
                return Some(depth+1);
            }
            else {
                return None;
            }
        }
        else {
            if occ.unwrap() != room as u8 {
                dirty = true;
            }
        }
    }
    
    if dirty {
        Some(0)
    }
    else {
        None
    }
}

pub fn top_incoming_room_depth(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), room: usize) -> Option<usize> {
    for i in 0..4 {
        let depth = 3 - i;
        let occ = state.1[room][depth];

        if occ.is_none() {
            return Some(depth);
        }
        else if occ.unwrap() != room as u8 {
            return None;
        }
    }
    None
}

pub fn find_min_path(state: &([Option<u8>; 7], [[Option<u8>; 4]; 4]), current_cost: usize) -> usize {
    let mut min_cost = usize::MAX;

    let mut solved = true;

    'win_check: for room in 0..4 {
        for depth in 0..4 {
            if state.1[room][depth] != Some(room as u8) {
                solved = false;
                break 'win_check;
            }
        }
    }

    if solved {
        return current_cost;
    }

    //println!("Solving from state {:?}", state);

    // Check moving out of room
    for room in 0..4 {
        // Moving out of [room]
        let depth = top_escape_room_depth(state, room);
        let min_available = mid_available_left(state, room);
        let max_available = mid_available_right(state, room);

        if depth.is_none() || min_available.is_none() || max_available.is_none() {
            //println!("  Couldn't move from room {} depth {:?} range {:?} to {:?}", room, depth, min_available, max_available);
            continue;
        }

        let depth = depth.unwrap();
        let min_available = min_available.unwrap();
        let max_available = max_available.unwrap();
        let value = state.1[room][depth].unwrap();
        
        // Look at all slots we can move to
        for new_slot in min_available..(max_available+1) {
            let mut new_state = *state;
            let move_cost = move_cost( room, depth, new_slot, value);

            assert_ne!(move_cost, 0);

            assert_eq!(new_state.0[new_slot], None);

            new_state.0[new_slot] = Some(value);
            new_state.1[room][depth] = None;

            //println!("  Considering move from room {} to slot {}", room, new_slot);

            let try_cost = find_min_path(&new_state, current_cost + move_cost);

            if try_cost < min_cost {
                min_cost = try_cost;
            }
        }
    }

    // Check moving into room
    for room in 0..4 {
        // Moving into [room]
        let depth = top_incoming_room_depth(state, room);
        let left_slot = first_full_left(state, room);
        let right_slot = first_full_right(state, room);

        if depth.is_none() {
            //println!("  Couldn't move into room {}", room);
            continue;
        }

        let depth = depth.unwrap();

        // Check left available slot
        if let Some(left) = left_slot {
            let value = state.0[left].unwrap();

            if value as usize == room {
                let mut new_state = *state;
                let move_cost = move_cost( room, depth, left, value);

                assert_ne!(move_cost, 0);

                //println!("  Considering move from slot {} to room {}", left, room);

                new_state.0[left] = None;
                new_state.1[room][depth] = Some(value);

                let try_cost = find_min_path(&new_state, current_cost + move_cost);

                if try_cost < min_cost {
                    min_cost = try_cost;
                }
            }
        }

        // Check right available slot
        if let Some(right) = right_slot {
            let value = state.0[right].unwrap();

            if value as usize == room {
                let mut new_state = *state;
                let move_cost = move_cost( room, depth, right, value);

                assert_ne!(move_cost, 0);

                //println!("  Considering move from slot {} to room {}", right, room);

                new_state.0[right] = None;
                new_state.1[room][depth] = Some(value);

                let try_cost = find_min_path(&new_state, current_cost + move_cost);

                if try_cost < min_cost {
                    min_cost = try_cost;
                }
            }
        }
    }

    min_cost
}

pub fn move_cost(room: usize, depth: usize, slot: usize, val: u8) -> usize {
    let mul = match val {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        _ => panic!(),
    };

    let horiz_cost = [[2, 1, 1, 3, 5, 7, 8],
    [4, 3, 1, 1, 3, 5, 6],
    [6, 5, 3, 1, 1, 3, 4],
    [8, 7, 5, 3, 1, 1, 2]];

    // Move to top
    let steps = depth + 1 + horiz_cost[room][slot];

    steps * mul
}

#[test]
pub fn test_room_depth() {
    //#############
    //#BB.C.C.C.BD#
    //###A#.#.#.###
    //  #A#C#.#.#
    //  #A#B#.#D#
    //  #A#D#.#D#
    //  #########
    let state1 = ([Some(1), Some(1), Some(2), Some(2), Some(2), Some(1), Some(3)], 
    [[Some(0), Some(0), Some(0), Some(0)],
    [None, Some(2), Some(1), Some(3)], 
    [None; 4], 
    [None, None, Some(3), Some(3)]]);

    assert_eq!(top_escape_room_depth(&state1, 0), None);
    assert_eq!(top_escape_room_depth(&state1, 1), Some(1));
    assert_eq!(top_escape_room_depth(&state1, 2), None);
    assert_eq!(top_escape_room_depth(&state1, 3), None);

    assert_eq!(top_incoming_room_depth(&state1, 0), None);
    assert_eq!(top_incoming_room_depth(&state1, 1), None);
    assert_eq!(top_incoming_room_depth(&state1, 2), Some(3));
    assert_eq!(top_incoming_room_depth(&state1, 3), Some(1));
}
