use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> isize {
    // ABC is Rock Paper Scissors
    // XYZ is Rock Paper Scissors
    input
        .lines()
        .map(|l| {
            let them = (l.as_bytes()[0] - 'A' as u8) as isize; // ABC
            let us = (l.as_bytes()[2] - 'X' as u8) as isize; // XYZ
            let win = (us - them + 1 + 3) % 3; // if they're paper/1 and we're rock/0, score 0 for loss, 1 for draw, 2 for win
                                               // Us item score plus win score
            (us + 1) + win * 3
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> isize {
    // ABC is Rock Paper Scissors
    // XYZ is Rock Paper Scissors
    input
        .lines()
        .map(|l| {
            let them = (l.as_bytes()[0] - 'A' as u8) as isize; // ABC
            let win = (l.as_bytes()[2] - 'X' as u8) as isize; // XYZ
                                                              //let win = (us - them + 1 + 3) % 3; // if they're paper/1 and we're rock/0, score 0 for loss, 1 for draw, 2 for win
                                                              // If they're rock/0 and we want to lose/0, we want scissors/2
                                                              // If they're rock/0 and we want to draw/1, we want rock/1
            let us = (them + win + 2) % 3;
            // Us item score plus win score
            (us + 1) + win * 3
        })
        .sum()
}
