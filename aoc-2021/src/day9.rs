use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|l| l.trim().chars().map(|t| t.to_digit(10).unwrap() as usize).collect()).collect()
}

pub fn find_low_points(input: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let h = input.len();
    let w = input[0].len();

    let mut low_points: Vec<(usize, usize)> = Vec::new();

    println!( "(w,h) = ({},{})", w, h );

    for y in 0..h {
        for x in 0..w {
            let depth = input[y][x];

            let mut is_low = true;

            if x > 0 {
                if input[y][x-1] <= depth {
                    is_low = false;
                }
            }

            if x < w - 1 {
                if input[y][x+1] <= depth {
                    is_low = false;
                }
            }

            if y > 0 {
                if input[y-1][x] <= depth {
                    is_low = false;
                }
            }

            if y < h - 1 {
                if input[y+1][x] <= depth {
                    is_low = false;
                }
            }

            if is_low {
                low_points.push((x, y));
            }
        }
    }

    low_points
}


#[aoc(day9, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    let low_points = find_low_points( input );

    low_points.iter().map(|(x, y)| input[*y][*x] + 1).sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Vec<usize>]) -> usize {

    let h = input.len();
    let w = input[0].len();

    let low_points = find_low_points( input );

    let num_basins = low_points.len();

    let mut basins = vec![vec![None; w]; h];

    for (b, (x, y)) in low_points.iter().enumerate() {
        basins[*y][*x] = Some(b);
    }

    for depth in 1..9 {
        loop {
            let mut found = 0;

            for y in 0..h {
                for x in 0..w {
                    if input[y][x] != depth {
                        continue;
                    }

                    if basins[y][x].is_some() {
                        continue;
                    }

                    let mut basin = None;

                    if x > 0 {
                        basin = basin.or(basins[y][x-1]);
                    }

                    if x < 99 {
                        basin = basin.or(basins[y][x+1]);
                    }

                    if y > 0 {
                        basin = basin.or(basins[y-1][x]);
                    }

                    if y < 99 {
                        basin = basin.or(basins[y+1][x]);
                    }

                    if basin.is_some() {
                        found += 1;
                    }

                    basins[y][x] = basin;
                }
            }

            if found == 0 {
                break;
            }
        }
    }

    let mut basin_sizes = Vec::new();

    for b in 0..num_basins {
        basin_sizes.push(basins.iter().map(|row| row.iter()).flatten().filter(|tile| if tile.is_some() { tile.unwrap() == b } else { false }).count());
    }

    //draw( &basins );

    basin_sizes.sort();

    //println!( "num_basins = {}, basin sizes = {:?}", num_basins, basin_sizes );

    basin_sizes[num_basins-1] * basin_sizes[num_basins-2] * basin_sizes[num_basins-3]
}


pub fn draw( map: &[Vec<Option<usize>>]) {
    const EMPTY: char = '.';

    const BASINS: &'static str = "0123456789!@#$%^&*()-=_+qwertyuiopasdfghjklzxcvbnm<>[]{}\\/?':~;";

    let nmod = BASINS.len();

    for y in 0..map.len() {
        println!( "{}", map[y].iter().map(|t| t.map_or(EMPTY, |b| BASINS.chars().nth(b % nmod).unwrap())).collect::<String>());
    }
}