use aoc_runner_derive::aoc;
use std::collections::hash_map::{HashMap, Entry};
use crate::util;


pub fn input_generator(input: &str) -> (Vec<u8>, Vec<(u8, u8, u8)>) {
    let mut lines = input.lines();

    let polymer = lines.next().unwrap().bytes().collect();

    let _blank = lines.next();

    let rules = lines
    .map(|l| {
        let mut it = l.trim().split(" -> ");
        let mut split = it.next().unwrap().bytes();
        (
            split.next().unwrap(),
            split.next().unwrap(),
            it.next().unwrap().bytes().next().unwrap(),
        )
    })
    .collect();

    ( polymer, rules )
}

pub fn grow( polymer: &[u8], rules: &[(u8, u8, u8)] ) -> Vec<u8> {
    let mut new = Vec::new();

    for (first, second) in polymer.iter().zip(polymer.iter().skip(1)) {
        new.push(*first);

        for (r1, r2, r3) in rules {
            if *r1 == *first && *r2 == *second {
                new.push(*r3);
                break;
            }
        }
    }

    new.push(polymer[polymer.len()-1]);

    new
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (mut polymer, rules) = input_generator( input );

    for _i in 0..10 {
        polymer = grow( &polymer, &rules );
    }

    let mut ocr: HashMap<u8, usize> = HashMap::new();

    for item in polymer {
        util::hash_increment( &mut ocr, item, 1 );
    }

    let mut ocr: Vec<usize> = ocr.iter().map(|(_key, value)| *value).collect();

    ocr.sort();

    ocr[ocr.len()-1] - ocr[0]
}

pub fn input_generator2(input: &str) -> (String, HashMap<String, (String, String, char)>) {
    let mut lines = input.lines();

    let polymer = lines.next().unwrap().to_string();

    let _blank = lines.next();

    let mut rules = HashMap::new();

    for line in lines {
        let mut it = line.trim().split(" -> ");

        let from_pat = it.next().unwrap().to_string();

        let out_char = it.next().unwrap().chars().next().unwrap();

        let to_pat1 = std::iter::once(from_pat.chars().next().unwrap()).chain(std::iter::once(out_char)).collect::<String>();
        let to_pat2 = std::iter::once(out_char).chain(std::iter::once(from_pat.chars().skip(1).next().unwrap())).collect::<String>();

        rules.insert(from_pat, (to_pat1, to_pat2, out_char));
    }
    
    (polymer, rules)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (poly, rules) = input_generator2( input );

    let mut counts: HashMap<char, usize> = HashMap::new();

    for c1 in poly.chars() {
        util::hash_increment(&mut counts, c1, 1);
    }

    let mut pat_counts: HashMap<String, usize> = HashMap::new();

    for (first, second) in poly.chars().zip(poly.chars().skip(1)) {
        let pat = std::iter::once(first).chain(std::iter::once(second)).collect::<String>();

        match pat_counts.entry(pat) {
            Entry::Occupied(o) => *o.into_mut() += 1,
            Entry::Vacant(v) => *v.insert(0) += 1,
        };
    }

    for _step in 0..40 {
        let mut next_pat_counts: HashMap<String, usize> = HashMap::new();

        for (old_pat, count) in pat_counts.iter() {
            let rule = rules.get(old_pat);

            if rule.is_some() {
                let (new_pat1, new_pat2, new_char) = rule.unwrap();

                util::hash_increment(&mut next_pat_counts, new_pat1.to_string(), *count);
                util::hash_increment(&mut next_pat_counts, new_pat2.to_string(), *count);
                util::hash_increment(&mut counts, *new_char, *count);
            }
        }

        pat_counts = next_pat_counts;
    }

    let mut counts: Vec<usize> = counts.iter().map(|(_key, value)| *value).collect();

    counts.sort();

    counts[counts.len()-1] - counts[0]
}
