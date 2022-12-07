use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_string()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input.iter().map(|line| {
        match check_valid( &line ) {
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            Ok(_) => 0,
            _ => panic!(),
        }
    }).sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    let mut scores: Vec<usize> = input.iter().map(|line| {
        match check_valid( &line ) {
            Err(_) => 0,
            Ok(stack) => {
                stack.iter().rev().fold(0, |points, next| points * 5 + completion_points(*next))
            },
        }
    }).filter(|score| *score != 0).collect();

    //println!( "{:?}", scores );

    scores.sort();

    scores[(scores.len()-1)/2]
}

pub fn is_opener(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

pub fn is_closer(c: char) -> bool {
    c == ')' || c == ']' || c == '}' || c == '>'
}

pub fn expected_opener(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

pub fn expected_closer(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

pub fn completion_points(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!(),
    }
}



// None for valid, Some(first_invalid_char) otherwise
pub fn check_valid(chunk: &str) -> Result<Vec<char>, char> {
    let mut stack: Vec<char> = Vec::new();

    for c in chunk.chars() {
        if is_opener( c ) {
            stack.push( c );
        }
        else if is_closer( c ) {
            if stack.is_empty() {
                // Invalid, return it
                return Err(c);
            }
            else {
                let should_open = stack.pop().unwrap();

                if should_open != expected_opener(c) {
                    return Err(c);
                }
            }
        }
        else {
            panic!();
        }
    }

    Ok(stack)
}

#[test]
pub fn test_part2() {
    let input = vec!["[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),];
    
    println!( "{}", solve_part2( &input ));
}