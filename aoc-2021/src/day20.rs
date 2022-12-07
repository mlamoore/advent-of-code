use aoc_runner_derive::aoc;


pub fn input_generator(input: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut lines = input.lines();

    let algorithm = lines.next().unwrap().chars().map(|char| char == '#').collect();

    let _blank = lines.next();

    let image = lines
    .map(|l| {
        l.trim().chars().map(|pix| pix == '#').collect()
    })
    .collect();

    ( algorithm, image )
}

pub fn pad_img(img: &mut Vec<Vec<bool>>, outside: bool) {
    for i in 0..img.len() {
        img[i].insert(0, outside);
        img[i].insert(1, outside);
        img[i].insert(2, outside);
        img[i].push(outside);
        img[i].push(outside);
        img[i].push(outside);
    }

    let width = img[0].len();
    let new_row = vec![outside; width];
    img.insert(0, new_row.clone());
    img.insert(1, new_row.clone());
    img.insert(2, new_row.clone());
    img.push(new_row.clone());
    img.push(new_row.clone());
    img.push(new_row);
}

pub fn enhance(algo: &[bool], img: &[Vec<bool>], outside: bool) -> (Vec<Vec<bool>>, bool) {
    let owidth = img[0].len();
    let nwidth = owidth+2;
    let oheight = img.len();
    let nheight = oheight+2;

    let noutside = algo[if outside { 511 } else { 0 }];

    let mut new = vec![vec![noutside; nwidth]; nheight];

    for oy in 1..oheight-1 {
        for ox in 1..owidth-1 {
            let idx = if img[oy-1][ox-1] { 1<<8 } else { 0 } + 
                if img[oy-1][ox] { 1<<7 } else { 0 } + 
                if img[oy-1][ox+1] { 1<<6 } else { 0 } + 
                if img[oy][ox-1] { 1<<5 } else { 0 } + 
                if img[oy][ox] { 1<<4 } else { 0 } + 
                if img[oy][ox+1] { 1<<3 } else { 0 } + 
                if img[oy+1][ox-1] { 1<<2 } else { 0 } + 
                if img[oy+1][ox] { 1<<1 } else { 0 } + 
                if img[oy+1][ox+1] { 1<<0 } else { 0 };

            new[oy+1][ox+1] = algo[idx];
        }
    }

    (new, noutside)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (algo, mut img) = input_generator(input);

    pad_img(&mut img, false);

    let mut outside = false;
    
    for _i in 0..2 {
        let (nimg, noutside) = enhance(&algo, &img, outside);

        img = nimg;
        outside = noutside;
    }
    
    println!( "Round 2 image (outside = {}): ", outside);
    display(&img);

    img.iter().map(|row| row.iter().fold(0, |s,p| if *p { s + 1 } else { s })).sum()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (algo, mut img) = input_generator(input);

    pad_img(&mut img, false);

    let mut outside = false;
    
    for _i in 0..50 {
        let (nimg, noutside) = enhance(&algo, &img, outside);

        img = nimg;
        outside = noutside;
    }
    
    println!( "Round 50 image (outside = {}): ", outside);
    display(&img);

    img.iter().map(|row| row.iter().fold(0, |s,p| if *p { s + 1 } else { s })).sum()
}

pub fn display(img: &[Vec<bool>]) {

    for row in img.iter() {
        println!("{}", row.iter().map(|p| if *p { '#' } else { '.' }).collect::<String>());
    }
}
