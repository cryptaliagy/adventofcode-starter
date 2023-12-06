// Remove the following line after you've implemented the functions.
#![allow(unused_variables)]

pub fn part_one(input: &str) -> u64 {
    142
}

pub fn part_two(input: &str) -> u64 {
    281
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "#
        .trim()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(test_input()), 142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(test_input()), 281);
    }
}
