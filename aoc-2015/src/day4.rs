use aoc_runner_derive::{aoc, aoc_generator};


#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut answer = 0;

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    }
}
