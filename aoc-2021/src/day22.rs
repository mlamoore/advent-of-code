use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<(bool, isize, isize, isize, isize, isize, isize)> {
    input
        .lines()
        .map(|line| {
            let (action, rest) = line.split_once(" x=").unwrap();
            let (xmin, rest) = rest.split_once("..").unwrap();
            let (xmax, rest) = rest.split_once(",y=").unwrap();
            let (ymin, rest) = rest.split_once("..").unwrap();
            let (ymax, rest) = rest.split_once(",z=").unwrap();
            let (zmin, zmax) = rest.split_once("..").unwrap();
            (
                action == "on",
                xmin.parse().unwrap(),
                xmax.parse().unwrap(),
                ymin.parse().unwrap(),
                ymax.parse().unwrap(),
                zmin.parse().unwrap(),
                zmax.parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &[(bool, isize, isize, isize, isize, isize, isize)]) -> usize {
    let mut state = [[[false; 101]; 101]; 101];

    for (set_state, xmin, xmax, ymin, ymax, zmin, zmax) in input {
        if *xmin < -50 || 50 < *xmax || *ymin < -50 || 50 < *ymax || *zmin < -50 || 50 < *zmax {
            continue;
        }

        for z in *zmin..*zmax+1 {
            for y in *ymin..*ymax+1 {
                for x in *xmin..*xmax+1 {
                    state[(z+50) as usize][(y+50) as usize][(x+50) as usize] = *set_state;
                }
            }
        }
    }

    state.iter().flat_map(|plane| plane.iter().flat_map(|row| row.iter().filter(|pix| **pix))).count()
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &[(bool, isize, isize, isize, isize, isize, isize)]) -> usize {
    let mut x_boundaries = Vec::new();
    let mut y_boundaries = Vec::new();
    let mut z_boundaries = Vec::new();

    for (_set_state, xmin, xmax, ymin, ymax, zmin, zmax) in input {
        x_boundaries.push(*xmin);
        x_boundaries.push(*xmax+1);
        y_boundaries.push(*ymin);
        y_boundaries.push(*ymax+1);
        z_boundaries.push(*zmin);
        z_boundaries.push(*zmax+1);
    }

    x_boundaries.sort();
    y_boundaries.sort();
    z_boundaries.sort();

    x_boundaries.dedup();
    y_boundaries.dedup();
    z_boundaries.dedup();

    let xsegs = x_boundaries.len();
    let ysegs = y_boundaries.len();
    let zsegs = z_boundaries.len();

    let mut state = vec![vec![vec![false; xsegs]; ysegs]; zsegs];

    for (set_state, xmin, xmax, ymin, ymax, zmin, zmax) in input {
        let xmin = index_of(&x_boundaries, *xmin);
        let xmax = index_of(&x_boundaries, *xmax+1);
        let ymin = index_of(&y_boundaries, *ymin);
        let ymax = index_of(&y_boundaries, *ymax+1);
        let zmin = index_of(&z_boundaries, *zmin);
        let zmax = index_of(&z_boundaries, *zmax+1);
        
        for z in zmin..zmax {
            for y in ymin..ymax {
                for x in xmin..xmax {
                    state[z][y][x] = *set_state;
                }
            }
        }
    }

    let mut volume = 0;

    for zi in 0..zsegs-1 {
        for yi in 0..ysegs-1 {
            for xi in 0..xsegs-1 {
                if state[zi][yi][xi] {
                    let x1 = x_boundaries[xi];
                    let x2 = x_boundaries[xi+1];
                    let y1 = y_boundaries[yi];
                    let y2 = y_boundaries[yi+1];
                    let z1 = z_boundaries[zi];
                    let z2 = z_boundaries[zi+1];
                    
                    volume += ((x2-x1) * (y2-y1) * (z2-z1)) as usize;
                }
            }
        }
    }

    volume
}

fn index_of(array: &[isize], value: isize) -> usize {
    for i in 0..array.len() {
        if array[i] >= value {
            return i;
        }
    }
    panic!();
}


