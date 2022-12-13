use aoc_runner_derive::{aoc, aoc_generator};


#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut answer = 0;

    loop {
        let digest = md5::compute(format!("{}{}", input, answer).as_bytes());

        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
            return answer;
        }

        answer += 1;
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut answer = 0;

    loop {
        let digest = md5::compute(format!("{}{}", input, answer).as_bytes());

        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return answer;
        }

        answer += 1;
    }
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
