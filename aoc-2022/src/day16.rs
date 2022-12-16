use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Valve {
    name: String,
    flow: usize,
    distances: Vec<usize>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> (Vec<Valve>, usize) {
    let mut input = input.to_owned();
    input.push('\n');

    let p = parser!(lines("Valve " string(upper upper) " has flow rate=" usize {"; tunnels lead to valves ", "; tunnel leads to valve "} repeat_sep(string(upper upper), ", ")));

    let valves = p.parse(&input).unwrap();

    let mut indices = HashMap::new();
    let mut output = Vec::new();

    let numvalves = valves.len();

    for (i, (valve, flow, _, _next_valves)) in valves.iter().enumerate() {
        indices.insert(valve.clone(), i);
        output.push(Valve {
            name: valve.clone(),
            flow: *flow,
            distances: vec![usize::MAX; numvalves],
        });
        output[i].distances[i] = 0;
    }

    for depth in 0.. {
        for ivalve in 0..numvalves {
            for inext in 0..numvalves {
                if output[ivalve].distances[inext] == depth {
                    // ivalve to inext is depth, find neighbors of inext and they are depth+1 from ivalve
                    for iconn in 0..valves[inext].3.len() {
                        let iconn = indices[&valves[inext].3[iconn]];

                        if output[iconn].distances[ivalve] == usize::MAX {
                            output[iconn].distances[ivalve] = depth + 1;
                            output[ivalve].distances[iconn] = depth + 1;
                        }
                    }
                }
            }
        }

        if output
            .iter()
            .all(|valve| valve.distances.iter().all(|d| *d != usize::MAX))
        {
            break;
        }
    }

    (output, indices["AA"])
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &(Vec<Valve>, usize)) -> usize {
    let valves = &input.0;
    let aa = input.1;
    let mut visited = vec![false; valves.len()];

    for i in 0..valves.len() {
        if valves[i].flow == 0 {
            visited[i] = true;
        }
    }

    find_fast_best(valves, aa, 30, &visited)
}

pub fn find_fast_best(
    input: &[Valve],
    current: usize,
    remaining: usize,
    visited: &[bool],
) -> usize {
    if !visited[current] {
        let mut visited = visited.to_vec();
        visited[current] = true;
        let relief = input[current].flow * (remaining - 1);
        let remaining = if relief == 0 {
            remaining
        } else {
            remaining - 1
        };
        find_fast_best(input, current, remaining, &visited) + relief
    } else if remaining <= 2 {
        0
    } else {
        let mut best = 0;

        for (next, distance) in input[current].distances.iter().enumerate() {
            if next != current && !visited[next] && distance + 1 < remaining {
                let poss = find_fast_best(input, next, remaining - distance, visited);

                if poss > best {
                    best = poss;
                }
            }
        }

        best
    }
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &(Vec<Valve>, usize)) -> usize {
    let valves = &input.0;
    let aa = input.1;
    let mut visited = vec![false; valves.len()];

    for i in 0..valves.len() {
        if valves[i].flow == 0 {
            visited[i] = true;
        }
    }

    find_parallel_best(valves, [aa, aa], [0, 0], 26, &visited)
}

pub fn find_parallel_best(
    input: &[Valve],
    current: [usize; 2],
    delay: [usize; 2],
    remaining: usize,
    visited: &[bool],
) -> usize {
    if remaining <= 2 {
        0
    } else {
        let who = if delay[0] == 0 { 0 } else { 1 };

        let mut best = 0;

        for (next, distance) in input[current[who]].distances.iter().enumerate() {
            if next != current[who] && !visited[next] && distance + 1 < remaining {
                let mut visited = visited.to_vec();
                visited[next] = true;
                let mut delay = delay.clone();
                delay[who] = distance + 1;
                let mut current = current.clone();
                current[who] = next;
                let relief = input[next].flow * (remaining - distance - 1);
                let time_passed = delay[0].min(delay[1]);
                delay[0] -= time_passed;
                delay[1] -= time_passed;

                let poss =
                    find_parallel_best(input, current, delay, remaining - time_passed, &visited)
                        + relief;

                if poss > best {
                    best = poss;
                }
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        );

        assert_eq!(solve_part1(&input), 1651);
    }
}
