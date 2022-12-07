use aoc_runner_derive::{aoc, aoc_generator};


#[derive(Debug,Clone, Eq, PartialEq)]
pub struct SnailNum {
    first: SnailElement,
    second: SnailElement,
}

#[derive(Debug,Clone, Eq, PartialEq)]
pub enum SnailElement {
    Literal(usize),
    Nested(Box<SnailNum>),
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<SnailNum> {
    input.lines().map(|line| parse_helper(line.trim()).0).collect()
}

// Returns 
fn parse_helper(input: &str) -> (SnailNum, &str) {
    let mut input = input;
    assert_eq!(input.as_bytes()[0], '[' as u8);

    input = input.split_at(1).1;

    let first = if input.as_bytes()[0] == '[' as u8 {
        // nested SnailNum
        let (subelement, remainder) = parse_helper(input);

        assert_eq!(remainder.as_bytes()[0], ',' as u8);

        input = remainder.split_at(1).1;

        SnailElement::Nested(Box::new(subelement))
    }
    else {
        // literal
        let (literal, remainder) = input.split_once(',').unwrap();

        input = remainder;

        SnailElement::Literal(literal.parse().unwrap())
    };


    let second = if input.as_bytes()[0] == '[' as u8 {
        // nested SnailNum
        let (subelement, remainder) = parse_helper(input);

        input = remainder.split_at(1).1;

        SnailElement::Nested(Box::new(subelement))
    }
    else {
        // literal
        let (literal, remainder) = input.split_once(']').unwrap();

        input = remainder;

        SnailElement::Literal(literal.parse().unwrap())
    };

    (SnailNum { first, second }, input)
}

pub fn magnitude(num: SnailNum) -> usize {
    let first_mag = match num.first {
        SnailElement::Literal(n) => n,
        SnailElement::Nested(n) => magnitude(*n),
    };
    let second_mag = match num.second {
        SnailElement::Literal(n) => n,
        SnailElement::Nested(n) => magnitude(*n),
    };

    3 * first_mag + 2 * second_mag
}

pub fn reduce(mut num: SnailNum) -> SnailNum {
    loop {
        let (explode_result, _, _, exploded) = explode_helper(SnailElement::Nested(Box::new(num.clone())), 0);

        if exploded {
            num = if let SnailElement::Nested(n) = explode_result { *n } else { panic!() };
            continue;
        }

        let (split_result, split) = split_helper(SnailElement::Nested(Box::new(num.clone())));

        if split {
            num = if let SnailElement::Nested(n) = split_result { *n } else { panic!() };
            continue;
        }

        break;
    }

    num
}

#[test]
fn test_explode() {
    let ex1 = input_generator("[[[[[9,8],1],2],3],4]")[0].clone();
    let ex1_exploded = input_generator("[[[[0,9],2],3],4]")[0].clone();

    let ex2 = input_generator("[7,[6,[5,[4,[3,2]]]]]")[0].clone();
    let ex2_exploded = input_generator("[7,[6,[5,[7,0]]]]")[0].clone();

    assert_ne!(ex1, ex1_exploded);
    assert_eq!(ex1, ex1);

    let (ex1r, lovrf, rovrf, chgd) = explode_helper(SnailElement::Nested(Box::new(ex1)), 0);

    assert_eq!(ex1r, SnailElement::Nested(Box::new(ex1_exploded)));
    assert_eq!(lovrf, 9);
    assert_eq!(rovrf, 0);
    assert_eq!(chgd, true);

    let ex2r = explode_helper(SnailElement::Nested(Box::new(ex2)), 0).0;
    assert_eq!( ex2r, SnailElement::Nested(Box::new(ex2_exploded)));


}

// Returns new number, left remainder, right remainder, whether a change happened
fn explode_helper(num: SnailElement, depth: usize) -> (SnailElement, usize, usize, bool) {
    match num {
        SnailElement::Literal(_) => (num, 0, 0, false),
        SnailElement::Nested(mut next) => {
            if depth == 4 {
                let first = if let SnailElement::Literal(f) = next.first { f } else { panic!(); };
                let second = if let SnailElement::Literal(s) = next.second { s } else { panic!(); };
                (SnailElement::Literal(0), first, second, true)
            }
            else {
                // Didn't explode at this level, recursively keep looking
                let (first_res, first_lrem, first_rrem, first_changed) = explode_helper(next.first, depth+1);

                next.first = first_res;

                if first_changed {
                    //println!( "First exploded at depth {}...", depth );
                    // Found our explosion, propagate it
                    // Can immediately propagate to right, need to pass left up the stack
                    if first_rrem != 0 {
                        //println!( "Propagating explosion right...");
                        next.second = explode_right_helper(next.second, first_rrem);
                    }
                    
                    (SnailElement::Nested(next), first_lrem, 0, true)
                }
                else {
                    let (second_res, second_lrem, second_rrem, second_changed) = explode_helper(next.second, depth+1);

                    next.second = second_res;

                    if second_changed {
                        //println!( "Second exploded at depth {}...", depth );
                        // Found our explosion, propagate it
                        // Can immediately propagate to left, need to pass right up the stack
                        if second_lrem != 0 {
                            //println!( "Propagating explosion left...");
                            next.first = explode_left_helper(next.first, second_lrem);
                        }
                        
                        (SnailElement::Nested(next), 0, second_rrem, true)
                    }
                    else {
                        (SnailElement::Nested(next), 0, 0, false)
                    }
                }
            }
        }
    }
}

fn explode_left_helper(num: SnailElement, leftward: usize) -> SnailElement {
    // explosion is happening right to left
    match num {
        SnailElement::Literal(n) => SnailElement::Literal(n+leftward),
        SnailElement::Nested(mut next) => {
            next.second = explode_left_helper(next.second, leftward);
            SnailElement::Nested(next)
        }
    }
}

fn explode_right_helper(num: SnailElement, rightward: usize) -> SnailElement {
    // explosion is happening left to right
    match num {
        SnailElement::Literal(n) => SnailElement::Literal(n+rightward),
        SnailElement::Nested(mut next) => {
            next.first = explode_right_helper(next.first, rightward);
            SnailElement::Nested(next)
        }
    }
}

fn split_helper(num: SnailElement) -> (SnailElement, bool) {
    match num {
        SnailElement::Literal(n) => {
            if n > 9 {
                (SnailElement::Nested(Box::new(SnailNum { first: SnailElement::Literal(n / 2), second: SnailElement::Literal(n - n / 2), })), true)
            }
            else {
                (num, false)
            }
        },
        SnailElement::Nested(next) => {
            let (first, changed) = split_helper(next.first);

            if changed {
                (SnailElement::Nested(Box::new(SnailNum { first: first.clone(), second: next.second })), true)
            }
            else {
                let (second, changed) = split_helper(next.second);
                // Change or no change, this will work

                (SnailElement::Nested(Box::new(SnailNum { first, second: second.clone() })), changed)
            }
        }
    }
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[SnailNum]) -> usize {
    let mut input = input.into_iter();

    let mut sum: SnailNum = input.next().unwrap().clone();

    for next in input {
        sum = SnailNum { first: SnailElement::Nested(Box::new(sum)), second: SnailElement::Nested(Box::new(next.clone())) };
        sum = reduce(sum);
    }
    magnitude(sum)
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[SnailNum]) -> usize {
    let mut highest_mag = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                let sum = SnailNum { first: SnailElement::Nested(Box::new(input[i].clone())), second: SnailElement::Nested(Box::new(input[j].clone())) };
                let mag = magnitude(reduce(sum));

                if mag > highest_mag {
                    highest_mag = mag;
                }
            }
        }
    }

    highest_mag
}
