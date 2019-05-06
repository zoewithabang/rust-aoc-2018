use crate::days::Day;
use std::fs;

pub struct Day5 {
    input: String,
}

impl Day5 {
    pub fn new() -> Day5 {
        let input = fs::read_to_string("res/day5.txt").unwrap();

        Day5 { input }
    }
}

impl Day for Day5 {
    fn part1(&self) -> String {
        "".to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}
