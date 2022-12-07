use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8, part1)]
pub fn input_generator(input: &str) -> Vec<[usize; 4]> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(" | ").skip(1).next().unwrap().split(' ');
            [
                it.next().unwrap().len(),
                it.next().unwrap().len(),
                it.next().unwrap().len(),
                it.next().unwrap().len(),
            ]
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[[usize; 4]]) -> usize {
    input
        .iter()
        .map(|a| a.iter())
        .flatten()
        .filter(|&&n| n == 2 || n == 4 || n == 3 || n == 7)
        .count()
}

pub fn ltr_to_bits(input: &str) -> u8 {
    input
        .chars()
        .map(|c| c.to_digit(36))
        .filter_map(|c| {
            if let Some(n) = c {
                if n >= 10 && n <= 16 {
                    Some(n - 10)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .fold(0, |n, d| n + (1 << d))
}

#[aoc_generator(day8, part2)]
pub fn input_generator2(input: &str) -> Vec<([u8; 10], [u8; 4])> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(" | ");

            let mut digits = it.next().unwrap().split(' ');
            let patterns = [
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
            ];

            let mut digits = it.next().unwrap().split(' ');
            let output = [
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
                ltr_to_bits(digits.next().unwrap()),
            ];

            (patterns, output)
        })
        .collect()
}

// For digits 0-10, which bits a-f should be set
static SEGMENTS: [u8; 10] = [
    0b1110111, 0b0100100, 0b1011101, 0b1101101, 0b0101110, 0b1101011, 0b1111011, 0b0100101,
    0b1111111, 0b1101111,
];

// Input: 10 scrambled patterns in any order
// Output: 10 scrambled patterns in order
pub fn unscramble(patterns: &[u8; 10]) -> [u8; 10] {
    // Find pattern indices
    let one = *patterns
        .iter()
        .filter(|&&d| d.count_ones() == 2)
        .next()
        .unwrap();
    let four = *patterns
        .iter()
        .filter(|&&d| d.count_ones() == 4)
        .next()
        .unwrap();
    let seven = *patterns
        .iter()
        .filter(|&&d| d.count_ones() == 3)
        .next()
        .unwrap();
    let eight = *patterns
        .iter()
        .filter(|&&d| d.count_ones() == 7)
        .next()
        .unwrap();

    // 6 segments: 0, 6, 9

    // 9: everything from 4 and 7, isn't 8
    let nine = *patterns
        .iter()
        .filter(|&&d| d != eight && (d & four) == four && (d & seven) == seven)
        .next()
        .unwrap();

    // 0: 6 segments, not 9, everything from 1
    let zero = *patterns
        .iter()
        .filter(|&&d| {
            d != nine && d.count_ones() == 6 && (d & one) == one
        })
        .next()
        .unwrap();

    // 6: 6 segments, not 0 or 9
    let six = *patterns
        .iter()
        .filter(|&&d| d != nine && d != zero && d.count_ones() == 6)
        .next()
        .unwrap();

    // 5 segments: 2, 3, 5
    // 3: subset of 9, everything from 1
    let three = *patterns
        .iter()
        .filter(|&&d| {
            d.count_ones() == 5 && (d & one) == one && (d & nine) == d
        })
        .next()
        .unwrap();

    // 2: 5 segments, contains segment 8 ^ 9
    let two = *patterns
        .iter()
        .filter(|&&d| {
            d.count_ones() == 5
                && (d & (eight ^ nine)) == (eight ^ nine)
        })
        .next()
        .unwrap();

    // 5: 5 segments, not 2 or 3
    let five = *patterns
        .iter()
        .filter(|&&d| d != two && d != three && d.count_ones() == 5)
        .next()
        .unwrap();

    [zero, one, two, three, four, five, six, seven, eight, nine]
}

pub fn convert(mapping: &[u8; 10], digit: u8) -> usize {
    mapping
        .iter()
        .enumerate()
        .filter(|(_i, &d)| d == digit)
        .map(|(i, _d)| i)
        .next()
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[([u8; 10], [u8; 4])]) -> usize {
    let mut total = 0;

    for (scrambled, output) in input {
        let mapping = unscramble(scrambled);

        let output = output
            .iter()
            .map(|d| convert(&mapping, *d))
            .fold(0, |s, d| s * 10 + d);

        total += output;
    }

    total
}

// Given a scrambled digit (7 segments) and a mapping indexed by scrambled bit outputting original bit, what is the original digit (binary number)
pub fn decode(mapping: &[u8; 7], digit: u8) -> usize {
    let d = digit;

    let _b = ((d >> 2) & 1) << mapping[2];

    let digit = (((d >> 0) & 1) << mapping[0])
        + (((d >> 1) & 1) << mapping[1])
        + (((d >> 2) & 1) << mapping[2])
        + (((d >> 3) & 1) << mapping[3])
        + (((d >> 4) & 1) << mapping[4])
        + (((d >> 5) & 1) << mapping[5])
        + (((d >> 6) & 1) << mapping[6]);

    //println!( "{} - {}", digit, b );

    for (i, &segs) in SEGMENTS.iter().enumerate() {
        if digit == segs {
            return i;
        }
    }

    panic!();
}
