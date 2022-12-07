use aoc_runner_derive::aoc;
use rand;


#[aoc(day21, part1)]
pub fn solve_part1(_input: &str) -> usize {

    let mut pos1 = 6;
    let mut pos2 = 3;

    let mut score1 = 0;
    let mut score2 = 0;
    let mut next_die_roll = 1;
    let mut num_die_rolls = 0;

    loop {
        let moves = 3*next_die_roll + 3;
        next_die_roll = ((next_die_roll + 3 - 1) % 100) + 1;
        num_die_rolls += 3;
        
        pos1 = ((pos1 + moves - 1) % 10) + 1;

        score1 += pos1;

        //println!( "Player 1 rolls {} and moves to space {} for a total score of {}; die has been rolled {} times.", moves, pos1, score1, num_die_rolls );

        if score1 >= 1000 {
            //println!( "Player 1 wins with {} points vs {} after {} die rolls", score1, score2, num_die_rolls );
            break score2 * num_die_rolls
        }

        let moves = 3*next_die_roll + 3;
        next_die_roll = ((next_die_roll + 3 - 1) % 100) + 1;
        num_die_rolls += 3;
        
        pos2 = ((pos2 + moves - 1) % 10) + 1;

        score2 += pos2;

        //println!( "Player 2 rolls {} and moves to space {} for a total score of {}; die has been rolled {} times.", moves, pos2, score2, num_die_rolls );


        if score2 >= 1000 {
            //println!( "Player 2 wins with {} points vs {} after {} die rolls", score2, score1, num_die_rolls );
            break score1 * num_die_rolls
        }


    }
}

#[aoc(day21, part2)]
pub fn solve_part2(_input: &str) -> usize {
    let final_wins = dirac_dice(0, [6, 3], [0, 0], true);
    //let final_wins = dirac_dice(0, [4, 8], [0, 0], true);

    if final_wins[0] > final_wins[1] {
        final_wins[0]
    }
    else {
        final_wins[1]
    }
}

pub fn dirac_dice( turn: usize, pos: [usize; 2], score: [usize; 2], print: bool ) -> [usize; 2] {
    // ROLL_FREQUENCIES[i] is how often a score of i will be rolled as the sum of 3 dirac dice rolls
    const ROLL_FREQUENCIES: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    let print_value = if print { rand::random::<usize>() % 7 + 3 } else { 0 };

    let mut wins = [0, 0];

    let next_turn = if turn == 0 { 1 } else { 0 };

    for roll_value in 3..10 {
        let roll_freq = ROLL_FREQUENCIES[roll_value];

        let mut new_pos = pos;
        let mut new_score = score;

        new_pos[turn] = ((new_pos[turn] + roll_value - 1) % 10) + 1;

        new_score[turn] += new_pos[turn];


        if roll_value == print_value {
            println!( "Player {} rolls {} and moves to space {} for a total score of {}.", turn, roll_value, new_pos[turn], new_score[turn] );
        }

        if new_score[turn] >= 21 {

            if roll_value == print_value {
                println!( "Player {} wins {} times.", turn, roll_freq );
            }
    
            wins[turn] += roll_freq;
        }
        else {
            let recursive_wins = dirac_dice(next_turn, new_pos, new_score, roll_value == print_value);


            if roll_value == print_value {
                println!( "After recursion players won {} vs {} times.", recursive_wins[0], recursive_wins[1] );
            }

            wins[0] += roll_freq * recursive_wins[0];
            wins[1] += roll_freq * recursive_wins[1];
        }
    }

    wins
}