use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|l| l.trim().chars().map(|t| t.to_digit(10).unwrap() as usize).collect()).collect()
}



pub fn bumpflash( energy: &mut [Vec<usize>], flashed: &mut [Vec<bool>], x: isize, y: isize) {
    if x >= 0 && x < energy[0].len() as isize && y >= 0 && y < energy.len() as isize {
        let x = x as usize;
        let y = y as usize;
        if !flashed[y][x] {
            energy[y][x] += 1;
            if energy[y][x] > 9 {
                flashed[y][x] = true;
                energy[y][x] = 0;

                for dy in -1..2 {
                    for dx in -1..2 {
                        if dx != 0 || dy != 0 {
                            bumpflash( energy, flashed, x as isize + dx, y as isize + dy);
                        }
                    }
                }
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    let w = input[0].len();
    let h = input.len();

    let mut energy: Vec<Vec<usize>> = input.to_vec();

    let mut flashes = 0;

    for _iteration in 0..100 {
        let mut flashed = vec![vec![false; w]; h];

        for y in 0..h {
            for x in 0..w {
                bumpflash( &mut energy, &mut flashed, x as isize, y as isize );
            }
        }

        flashes += flashed.iter().map(|row| row.iter()).flatten().filter(|&&cell| cell).count();
    }

    flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Vec<usize>]) -> usize {
    let w = input[0].len();
    let h = input.len();

    let mut energy: Vec<Vec<usize>> = input.to_vec();

    let mut step = 0;

    loop {
        step += 1;

        let mut flashed = vec![vec![false; w]; h];

        for y in 0..h {
            for x in 0..w {
                bumpflash( &mut energy, &mut flashed, x as isize, y as isize );
            }
        }

        if w * h == flashed.iter().map(|row| row.iter()).flatten().filter(|&&cell| cell).count() {
            break step
        }
    }
}

#[test]
pub fn test_part1() {
    let mut energy: Vec<Vec<usize>> = vec![
        vec![5,4,8,3,1,4,3,2,2,3 ],
        vec![2,7,4,5,8,5,4,7,1,1 ],
        vec![5,2,6,4,5,5,6,1,7,3 ],
        vec![6,1,4,1,3,3,6,1,4,6 ],
        vec![6,3,5,7,3,8,5,4,7,8 ],
        vec![4,1,6,7,5,2,4,6,4,5 ],
        vec![2,1,7,6,8,4,1,7,2,1 ],
        vec![6,8,8,2,8,8,1,1,3,4 ],
        vec![4,8,4,6,8,4,8,5,5,4 ],
        vec![5,2,8,3,7,5,1,5,2,6 ]
    ];

    let w = energy[0].len();
    let h = energy.len();

    let mut flashes = 0;

    for it in 0..10 {
        let mut flashed = vec![vec![false; w]; h];

        for y in 0..h {
            for x in 0..w {
                bumpflash( &mut energy, &mut flashed, x as isize, y as isize );
            }
        }

        println!( "Step {}", it+1 );
        for y in 0..h {
            println!( "    {:?}", energy[y] );
        }

        flashes += flashed.iter().map(|row| row.iter()).flatten().filter(|&&cell| cell).count();
    }

    println!( "{}", flashes );
}

#[test]
pub fn test_part1b() {
    let mut energy: Vec<Vec<usize>> = vec![
        vec![1,1,1,1,1],
        vec![1,9,9,9,1],
        vec![1,9,1,9,1],
        vec![1,9,9,9,1],
        vec![1,1,1,1,1]
    ];

    let w = energy[0].len();
    let h = energy.len();

    let mut flashes = 0;

    for it in 0..2 {
        let mut flashed = vec![vec![false; w]; h];

        for y in 0..h {
            for x in 0..w {
                bumpflash( &mut energy, &mut flashed, x as isize, y as isize );

                println!( "Step {} bump ({},{})", it+1, x, y );
                for py in 0..h {
                    println!( "    {:?}", energy[py] );
                }
            }
        }



        flashes += flashed.iter().map(|row| row.iter()).flatten().filter(|&&cell| cell).count();
    }

    println!( "{}", flashes );
}