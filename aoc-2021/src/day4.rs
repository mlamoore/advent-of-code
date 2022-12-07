use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<[[usize; 5]; 5]>) {
    let mut input = input.lines();

    let draws = input
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = Vec::new();

    loop {
        let mut board = [[0usize; 5]; 5];

        let line = input.next();

        if line.is_none() {
            break;
        }

        for i in 0..5 {
            let mut line = input.next().unwrap().split(' ');

            for j in 0..5 {
                let num = line.next().unwrap();

                let num = num.parse();

                if num.is_err() {
                    board[i][j] = line.next().unwrap().parse().unwrap();
                } else {
                    board[i][j] = num.unwrap();
                }
            }
        }

        boards.push(board);
    }

    (draws, boards)
}

pub fn check_win(marked: &[[bool; 5]; 5]) -> bool {
    // Diagonals aren't supposed to win! I would have saved tons of time if I'd read a bit more carefully!
    /*
    // Check forward diagonal
    let mut won = true;
    for i in 0..5 {
        if marked[i][i] == false {
            won = false;
            break;
        }
    }

    if won {
        return true;
    }

    // Check backward diagonal
    let mut won = true;
    for i in 0..5 {
        if marked[4-i][i] == false {
            won = false;
            break;
        }
    }

    if won {
        return true;
    }
    */
    // Check rows and columns
    for i in 0..5 {
        let mut won = true;

        for j in 0..5 {
            if marked[i][j] == false {
                won = false;
                break;
            }
        }

        if won {
            return true;
        }

        let mut won = true;

        for j in 0..5 {
            if marked[j][i] == false {
                won = false;
                break;
            }
        }

        if won {
            return true;
        }
    }

    false
}

pub fn calc_score(marked: &[[bool; 5]; 5], board: &[[usize; 5]; 5], draw: usize) -> usize {
    let mut score = 0;

    for i in 0..5 {
        for j in 0..5 {
            if marked[i][j] == false {
                score += board[i][j];
            }
        }
    }

    score * draw
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(Vec<usize>, Vec<[[usize; 5]; 5]>)) -> usize {
    let (draws, boards) = input;

    let mut marked = vec![[[false; 5]; 5]; boards.len()];

    for draw in draws {
        for (n, board) in boards.iter().enumerate() {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == *draw {
                        marked[n][i][j] = true;
                    }
                }
            }

            if check_win(&marked[n]) {
                return calc_score(&marked[n], board, *draw);
            }
        }
    }

    0
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<usize>, Vec<[[usize; 5]; 5]>)) -> usize {
    let (draws, boards) = input;

    let mut marked = vec![[[false; 5]; 5]; boards.len()];

    let mut remaining: Vec<usize> = (0..boards.len()).collect();

    let mut won: Vec<usize> = Vec::new();

    println!("Playing {} boards", boards.len());

    for draw in draws {
        //for (n, board) in boards.iter().enumerate() {
        for n in remaining.iter() {
            for i in 0..5 {
                for j in 0..5 {
                    if boards[*n][i][j] == *draw {
                        marked[*n][i][j] = true;
                    }
                }
            }

            if check_win(&marked[*n]) {
                if remaining.len() == 1 {
                    println!("Board {} won last, draw = {}", *n, *draw);
                    //println!( "{:#?}", marked[*n]);
                    //println!( "{:#?}", boards[*n]);
                    return calc_score(&marked[*n], &boards[*n], *draw);
                }
                println!(
                    "Board {} won, score = {}",
                    *n,
                    calc_score(&marked[*n], &boards[*n], *draw)
                );
                won.push(*n);
            }
        }

        for winner in won.iter() {
            for i in 0..remaining.len() {
                if remaining[i] == *winner {
                    remaining.remove(i);
                    println!("Removed board {}", *winner);
                    break;
                }
            }
        }

        won = Vec::new();
    }

    0
}

#[test]
pub fn verify_wins() {
    assert!(check_win(&[
        [true; 5], [false; 5], [false; 5], [false; 5], [false; 5]
    ]));
    assert!(check_win(&[
        [false; 5], [true; 5], [false; 5], [false; 5], [false; 5]
    ]));
    assert!(check_win(&[
        [false; 5], [false; 5], [true; 5], [false; 5], [false; 5]
    ]));
    assert!(check_win(&[
        [false; 5], [false; 5], [false; 5], [true; 5], [false; 5]
    ]));
    assert!(check_win(&[
        [false; 5], [false; 5], [false; 5], [false; 5], [true; 5]
    ]));

    assert!(
        false
            == check_win(&&[
                [true, false, false, false, false],
                [false, true, false, false, false],
                [false, false, true, false, false],
                [false, false, false, true, false],
                [false, false, false, false, true]
            ])
    );

    assert!(
        false
            == check_win(&&[
                [false, false, false, false, true],
                [false, false, false, true, false],
                [false, false, true, false, false],
                [false, true, false, false, false],
                [true, false, false, false, false]
            ])
    );

    assert!(check_win(&&[
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, false, false, false, false]
    ]));

    assert!(check_win(&&[
        [false, true, false, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false],
        [false, true, false, false, false]
    ]));

    assert!(check_win(&&[
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false]
    ]));

    assert!(check_win(&&[
        [false, false, false, true, false],
        [false, false, false, true, false],
        [false, false, false, true, false],
        [false, false, false, true, false],
        [false, false, false, true, false]
    ]));

    assert!(check_win(&&[
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true],
        [false, false, false, false, true]
    ]));
}
