use aoc_runner_derive::aoc;


#[aoc(day17, part1)]
pub fn solve_part1(_input: &str) -> isize {
    test_shots(230, 283, -107, -57).0
}

#[aoc(day17, part2)]
pub fn solve_part2(_input: &str) -> usize {
    test_shots(230, 283, -107, -57).1
}

#[test]
pub fn test_example() {
    assert_eq!( (45, 112), test_shots(20, 30, -10, -5) );
}

#[test]
pub fn test_detail() {
    assert_eq!(Some(45), find_yvel_hit(6, 20, 30, 9, -10, -5));
}

pub fn test_shots( xmin: isize, xmax: isize, ymin: isize, ymax: isize ) -> (isize, usize) {
    let mut tallest_height = 0;
    let mut num_shots = 0;

    let yvel_max = isize::max(isize::abs(ymin),isize::abs(ymax)) + 2;

    for yvel in -yvel_max..yvel_max {
        for xvel in 1..xmax+2 {
            if let Some(height) = find_yvel_hit( xvel, xmin, xmax, yvel, ymin, ymax) {
                //println!("  Reached height {} with velocity ({},{})", height, xvel, yvel);
                num_shots += 1;
                if height > tallest_height {
                    tallest_height = height;
                }
            }
        }
    }

    (tallest_height, num_shots)
}

pub fn find_yvel_hit( xvel: isize, xmin: isize, xmax: isize, yvel: isize, ymin: isize, ymax: isize) -> Option<isize> {
    let mut xvel = xvel;
    let mut xpos = 0;
    let mut yvel = yvel;
    let mut ypos = 0;
    let mut yposmax = 0;

    loop {
        ypos += yvel;
        xpos += xvel;
        yvel -= 1;
        if xvel > 0 {
            xvel -= 1;
        }

        //println!( "pos = ({},{}), vel = ({},{})", xpos, ypos, xvel, yvel);

        if xvel == 0 && xpos < xmin {
            return None;
        }

        if xpos > xmax {
            return None;
        }

        if ypos > yposmax {
            yposmax = ypos;
        }

        if ypos <= ymax {
            if ypos < ymin {
                return None;
            }
            else if xmin <= xpos && xpos <= xmax {
                return Some(yposmax)
            }
        }
    }
}