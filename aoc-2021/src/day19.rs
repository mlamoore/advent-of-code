use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Vec<(isize, isize, isize)>> {
    let mut scanners = Vec::new();

    let mut probes = Vec::new();

    for line in input.lines().skip(1) {
        if line == "" {
            // Do nothing
        }
        else if line.split_at(3).0 == "---" {
            scanners.push(probes);
            probes = Vec::new();
        }
        else {
            let mut coords = line.split(',');

            probes.push((coords.next().unwrap().parse().unwrap(), coords.next().unwrap().parse().unwrap(), coords.next().unwrap().parse().unwrap()));
        }
    }

    scanners.push(probes);

    scanners
}

pub fn transform(pt: (isize, isize, isize), rotation: usize, offset: (isize, isize, isize)) -> (isize, isize, isize) {
    let xrot = rotation & 3;
    let newdir = rotation >> 2;

    // First, rotate around x axis
    // x y z
    // x z -y
    // x -y -z
    // x -z y
    let pt = match xrot {
        0 => (pt.0, pt.1, pt.2),
        1 => (pt.0, pt.2, -pt.1),
        2 => (pt.0, -pt.1, -pt.2),
        3 => (pt.0, -pt.2, pt.1),
        _ => panic!(),
    };

    // Then, choose new forward dir
    // x y z
    // -x y -z
    // z y -x
    // -z y x
    // y -x z
    // -y x z
    let pt = match newdir {
        0 => (pt.0, pt.1, pt.2),
        1 => (-pt.0, pt.1, -pt.2),
        2 => (pt.2, pt.1, -pt.0),
        3 => (-pt.2, pt.1, pt.0),
        4 => (pt.1, -pt.0, pt.2),
        5 => (-pt.1, pt.0, pt.2),
        _ => panic!(),
    };

    (pt.0 + offset.0, pt.1 + offset.1, pt.2 + offset.2)
}

#[test]
fn test_test_pair_match() {
    let scn1 = vec![
        (404,-588,-901),
        (528,-643,409),
        (-838,591,734),
        (390,-675,-793),
        (-537,-823,-458),
        (-485,-357,347),
        (-345,-311,381),
        (-661,-816,-575),
        (-876,649,763),
        (-618,-824,-621),
        (553,345,-567),
        (474,580,667),
        (-447,-329,318),
        (-584,868,-557),
        (544,-627,-890),
        (564,392,-477),
        (455,729,728),
        (-892,524,684),
        (-689,845,-530),
        (423,-701,434),
        (7,-33,-71),
        (630,319,-379),
        (443,580,662),
        (-789,900,-551),
        (459,-707,401),
    ];

    let scn2 = vec![
        (686,422,578),
        (605,423,415),
        (515,917,-361),
        (-336,658,858),
        (95,138,22),
        (-476,619,847),
        (-340,-569,-846),
        (567,-361,727),
        (-460,603,-452),
        (669,-402,600),
        (729,430,532),
        (-500,-761,534),
        (-322,571,750),
        (-466,-666,-811),
        (-429,-592,574),
        (-355,545,-477),
        (703,-491,-529),
        (-328,-685,520),
        (413,935,-424),
        (-391,539,-444),
        (586,-435,557),
        (-364,-763,-893),
        (807,-499,-711),
        (755,-354,-619),
        (553,889,-390),
    ];

    let match_res = test_pair_match( &scn1, 0, (0, 0, 0), &scn2);

    println!("Test match results: {:?}", match_res);
}

// Returns Some((rotation, offset)) on success
pub fn test_pair_match( scn1: &[(isize, isize, isize)], rot1: usize, offset1: (isize, isize, isize), scn2: &[(isize, isize, isize)]) -> Option<(usize, (isize, isize, isize))> {
    for rot2 in 0..24 {
        for pb1 in 0..scn1.len() {
            'matching_loop: for pb2 in 0..scn2.len() {
                // Assume these probes are the same, check if that works
                let pb1loc = transform(scn1[pb1], rot1, offset1);
                let pb2loc = transform(scn2[pb2], rot2, (0, 0, 0));
                let offset2 = (pb1loc.0 - pb2loc.0, pb1loc.1 - pb2loc.1, pb1loc.2 - pb2loc.2);
                let mut num_matches = 1;

                //println!("===== Testing rot {} offset {:?} =====", rot2, offset2);

                for pb1ck in 0..scn1.len() {
                    if pb1ck != pb1 {
                        // Skip our key probe
                        let pb1loc = transform(scn1[pb1ck], rot1, offset1);

                        if isize::abs(pb1loc.0 - offset2.0) <= 1000 && isize::abs(pb1loc.1 - offset2.1) <= 1000 && isize::abs(pb1loc.2 - offset2.2) <= 1000 {
                            // We need a match
                            /*
                            let mut found_match = false;

                            for pt2ck in scn2.iter() {
                                let pt2ckloc = transform(*pt2ck, rot2, offset2);

                                if pt2ckloc == pb1ck
                            }
                            */

                            let lines_up = scn2.iter().map(|ptorig| transform(*ptorig, rot2, offset2)).filter(|pt| *pt == pb1loc).count();

                            match lines_up {
                                0 => { continue 'matching_loop; },
                                1 => { num_matches += 1; },
                                _ => panic!(),
                            };
                        }
                        else {
                            //println!("Out of range");
                        }
                    }
                }

                if num_matches >= 12 {
                    return Some((rot2, offset2));
                }
            }
        }
    }

    None
}

// stores (rotation, offset) of cooresponding scanners of input
pub fn align_scanners(input: &[Vec<(isize, isize, isize)>]) -> Vec<(usize, (isize, isize, isize))> {
    // stores scanner index of scanners that haven't been located yet
    let mut unmatched: Vec<usize> = (1..input.len()).collect();

    // stores (scanner_index, rotation, offset)
    let mut matched: Vec<(usize, usize, (isize, isize, isize))> = vec![(0, 0, (0, 0, 0))];

    'search_loop: while !unmatched.is_empty() {
        for baseline in 0..matched.len() {
            for candidate in 0..unmatched.len() {
                let base = matched[baseline];
                if let Some((rotation, offset)) = test_pair_match(&input[base.0], base.1, base.2, &input[unmatched[candidate]]) {
                    // Found a match!
                    //println!( "Match! Scanner {} lines up with scanner {} with rotation {} and offset {:?}", candidate, baseline, rotation, offset);
                    matched.push((unmatched[candidate], rotation, offset));
                    unmatched.remove(candidate);
                    continue 'search_loop;
                }
            }
        }

        // Should always find a match somewhere
        panic!();
    }

    //println!("{:?}", matched);

    let mut matched_ordered = vec![(0, (0, 0, 0)); input.len()];

    for (scanner, rotation, offset) in matched {
        matched_ordered[scanner] = (rotation, offset);
    }

    matched_ordered
}


#[aoc(day19, part1)]
pub fn solve_part1(input: &[Vec<(isize, isize, isize)>]) -> usize {

    //println!("Input: {:?}", input);

    // stores (scanner_index, rotation, offset)
    let matched = align_scanners(input);

    println!("Matched: {:?}", matched);

    let mut beacons = HashSet::new();

    for (i, (rotation, offset)) in matched.iter().enumerate() {
        for beacon in &input[i] {
            beacons.insert(transform(*beacon, *rotation, *offset));
        }
    }

    beacons.iter().count()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &[Vec<(isize, isize, isize)>]) -> isize {

    //println!("Input: {:?}", input);

    // stores (scanner_index, rotation, offset)
    let matched = align_scanners(input);

    println!("Matched: {:?}", matched);

    let mut max_distance = 0;

    for i in 0..matched.len() {
        for j in 0..matched.len() {
            let s1 = matched[i].1;
            let s2 = matched[j].1;
            let distance = isize::abs(s1.0 - s2.0) + isize::abs(s1.1 - s2.1) + isize::abs(s1.2 - s2.2);

            if distance > max_distance {
                max_distance = distance;
            }
        }
    }

    max_distance
}

