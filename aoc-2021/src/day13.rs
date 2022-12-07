use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    AlongX( usize ),
    AlongY( usize ),
}

pub fn input_generator(input: &str) -> (Vec<(usize, usize)>, Vec<Fold>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    for line in input.lines() {
        let mut coord_it = line.trim().split(',');
        let mut fold_it = line.trim().split('=');

        let xcoord = coord_it.next();
        let ycoord = coord_it.next();

        if let Some(ystr) = ycoord {
            points.push( (
                xcoord.unwrap().parse().unwrap(),
                ystr.parse().unwrap(),
            ) );
        }

        let axisstr = fold_it.next();
        let coordstr = fold_it.next();

        if let Some(coord) = coordstr {
            match axisstr.unwrap() {
                "fold along x" => folds.push(Fold::AlongX(coord.parse().unwrap())),
                "fold along y" => folds.push(Fold::AlongY(coord.parse().unwrap())),
                _ => panic!(),
            };
        }
    }
    
    (points, folds)
}

pub fn init_img( points: &[(usize, usize)] ) -> Vec<Vec<bool>> {
    let (width, height) = points.iter().fold((0, 0), |(xmax, ymax), (xnew, ynew)| (if *xnew > xmax { *xnew } else { xmax }, if *ynew > ymax { *ynew } else { ymax }) );

    let mut image = vec![vec![false; width+1]; height+1];

    for point in points {
        let (x, y) = *point;

        image[y][x] = true;
    }

    image
}

pub fn fold( image: &[Vec<bool>], instruction: Fold ) -> Vec<Vec<bool>> {
    let height = image.len();
    let width = image[0].len();

    let mut new_image = Vec::new();

    match instruction {
        Fold::AlongY( before ) => {
            // one line on, no dots there
            let after = height - before - 1;

            let new_height = usize::max(before, after);

            for i in 0..new_height {
                // before_index = i + before - new_height (could be negative)
                // after_index
                //      after bigger:
                //          i=0 -> height-1
                //          after_index = height-1-i
                //      after smaller:
                //          i=before-1 -> after_index = before+1
                //          after_index = before+1 + (before-1) - i
                //          after_index = 2*before-i (could be biggher than height-1)

                if i + before >= new_height {
                    new_image.push(image[i + before - new_height].clone());
                }
                else {
                    new_image.push(vec![false; width]);
                }

                let after_index = if after >= before { height - 1 - i } else { 2 * before - i };

                if after_index < height {
                    for x in 0..width {
                        new_image[i][x] = new_image[i][x] || image[after_index][x];
                    }
                }

            }
        },
        Fold::AlongX( before ) => {
            let after = width - before - 1;

            let new_width = usize::max(before, after);

            for y in 0..height {
                new_image.push(vec![false; new_width]);

                for i in 0..new_width {

                    if i + before >= new_width {
                        new_image[y][i] = image[y][i + before - new_width];
                    }

                    let after_index = if after >= before { width - 1 - i } else { 2 * before - i };

                    if after_index < width {
                        new_image[y][i] = new_image[y][i] || image[y][after_index];
                    }
                }
            }
        },
    }

    new_image
}


#[aoc(day13, part1)]
pub fn solve_part1(input: &str ) -> usize {
    let (points, folds) = input_generator( input );

    let image = init_img( &points );

    let new_image = fold( &image, folds[0] );

    
    new_image.iter().map(|row| row.iter()).flatten().filter(|pt| **pt).count()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str ) -> usize {
    let (points, folds) = input_generator( input );

    let mut image = init_img( &points );

    for next_fold in folds.iter() {
        image = fold( &image, *next_fold );
    }
    
    show_image( &image );

    42
}

#[test]
pub fn test_example() {
    let input = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5\n";

    let (points, folds) = input_generator( input );

    let image = init_img( &points );

    let count = image.iter().map(|row| row.iter()).flatten().filter(|pt| **pt).count();
    println!( "Image (count = {}):", count );
    show_image( &image );

    let image = fold( &image, folds[0] );

    let count = image.iter().map(|row| row.iter()).flatten().filter(|pt| **pt).count();
    println!( "Image after fold 1 (count = {}):", count );
    show_image( &image );

    let image = fold( &image, folds[1] );

    let count = image.iter().map(|row| row.iter()).flatten().filter(|pt| **pt).count();
    println!( "Image after fold 2 (count = {}):", count );
    show_image( &image );


}

pub fn show_image( image: &[Vec<bool>] ) {
    for row in image.iter() {
        for point in row.iter() {
            print!( "{}", if *point { "#" } else { "." } );
        }
        println!( "" );
    }
}