use aoc_runner_derive::{aoc, aoc_generator};
//use std::collections::{HashMap, BTreeMap};
//use std::collections::hash_set::HashSet;
//use crate::util;



#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<bool> {
    //hex::decode(input.trim().bytes().chain(std::iter::once(32).cycle().take(8)).collect::<Vec<u8>>()).unwrap()
    let idx = [
        [false, false, false, false], 
        [false, false, false,  true], 
        [false, false,  true, false], 
        [false, false,  true,  true], 
        [false,  true, false, false], 
        [false,  true, false,  true], 
        [false,  true,  true, false], 
        [false,  true,  true,  true], 
        [ true, false, false, false], 
        [ true, false, false,  true], 
        [ true, false,  true, false], 
        [ true, false,  true,  true], 
        [ true,  true, false, false], 
        [ true,  true, false,  true], 
        [ true,  true,  true, false], 
        [ true,  true,  true,  true]];
    
    input.trim().bytes().map(|b| if b >= 48 && b <= 57 { b - 48 } else if b >= 65 && b <= 70 { b + 10 - 65 } else { panic!(); } ).map(|b| idx[b as usize].iter().cloned()).flatten().collect()
}


#[aoc(day16, part1)]
pub fn solve_part1(input: &[bool]) -> usize {
    decode(input, 0).1
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[bool]) -> usize {
    decode(input, 0).0
}

#[test]
pub fn test1() {
    let input = input_generator("D2FE28");

    assert_eq!((2021, 6, 21), decode(&input, 0));
}

#[test]
pub fn test2() {
    let input = input_generator("38006F45291200");

    assert_eq!((1, 9, 7+15+27), decode(&input, 0));
}

#[test]
pub fn test3() {
    let input = input_generator("EE00D40C823060");

    assert_eq!((3, 14, 7+11+33), decode(&input, 0));
}

// Returns (value, version_sum, next_pos)
pub fn decode( input: &[bool], pos: usize ) -> (usize, usize, usize) {
    //println!( "New packet at {}", pos );
    let version = bitfield(input, pos, 3);
    let typeid = bitfield(input, pos+3, 3);

    //println!( "  version {}, typeid {}", version, typeid );

    if typeid == 4 {
        let mut pos = pos + 6;
        let mut value = 0;

        loop {
            let cont = bitfield(input, pos, 1);
            let chunk = bitfield(input, pos + 1, 4);

            //println!("    chunk at {}, val {}, cont {}", pos, chunk, cont);

            value *= 16;
            value += chunk;

            pos += 5;

            if cont == 0 {
                return (value, version, pos);
            }
        }
    }
    else {
        let lengthtype = bitfield(input, pos+6, 1);
        let mut versionsum = version;
        let mut pos = pos + 7;
        let mut subvalues = Vec::new();

        if lengthtype == 0 {
            let subpktbits = bitfield(input, pos, 15);

            //println!("  operator, lengthtype {}, length {} bits", lengthtype, subpktbits);

            pos += 15;
            let endpos = pos + subpktbits;

            while pos < endpos {
                //println!("    subpacket at {}", pos);
                let (nextvalue, nextsum, nextpos) = decode(input, pos);

                subvalues.push(nextvalue);
                versionsum += nextsum;
                pos = nextpos;
            }
        }
        else {
            let subpkts = bitfield(input, pos, 11);

            //println!("  operator, lengthtype {}, length {} packets", lengthtype, subpkts);

            pos += 11;
            
            for _subpkt in 0..subpkts {
                //println!("    subpacket at {}", pos);
                let (nextvalue, nextsum, nextpos) = decode(input, pos);

                subvalues.push(nextvalue);
                versionsum += nextsum;
                pos = nextpos;
            }
        }
        let value = eval_operator( typeid, &subvalues);

        //println!("  operator {} result {}", typeid, value);

        (value, versionsum, pos)
    }
}

pub fn eval_operator( op: usize, args: &[usize] ) -> usize {
    match op {
        0 => args.iter().sum(),
        1 => args.iter().fold(1, |prod, item| prod * *item),
        2 => *args.iter().min().unwrap(),
        3 => *args.iter().max().unwrap(),
        5 => if args[0] > args[1] { 1 } else { 0 },
        6 => if args[0] < args[1] { 1 } else { 0 },
        7 => if args[0] == args[1] { 1 } else { 0 },
        _ => panic!(),
    }
}

pub fn bitfield( input: &[bool], start: usize, bits: usize ) -> usize {

    let mut field = 0;

    for bit in 0..bits {
        field *= 2;

        field += if input[start+bit] { 1 } else { 0 };
    }

    field
}