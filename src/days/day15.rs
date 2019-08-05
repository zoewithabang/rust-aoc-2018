use crate::days::Day;
use std::fs;

pub struct Day15 {
    input: String,
}

impl Day15 {
    pub fn new() -> Day15 {
        let input = fs::read_to_string("res/day13.txt").unwrap();

        Day15 { input }
    }
}

impl Day for Day15 {
    fn part1(&self) -> String {
        format!("")
    }

    fn part2(&self) -> String {
        format!("")
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day15::new().part1().ends_with(""));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day15::new().part2().ends_with(""));
    }
}
