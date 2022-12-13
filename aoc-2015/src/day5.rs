use aoc_runner_derive::{aoc, aoc_generator};


#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .iter()
        .filter(|name| name.chars().filter(|ch| "aeiou".contains(ch)).count() >= 3)
        .filter(|name| name.chars().zip(name.chars().skip(1)).any(|(c1, c2)| c1 == c2))
        .filter(|name| name.chars().zip(name.chars().skip(1)).any(|(c1, c2)| c1 == c2))
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
        assert_eq!(digest[0], 0xc3);
    }
}
