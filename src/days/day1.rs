use crate::days::Day;
use std::collections::HashSet;
use std::fs;

pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new() -> Day1 {
        let input = fs::read_to_string("res/day1.txt").unwrap();

        Day1 { input }
    }
}

impl Day for Day1 {
    fn part1(&self) -> String {
        let result = &self
            .input
            .lines()
            .map(|value| value.parse::<i32>().unwrap())
            .sum::<i32>();

        return format!("Resulting frequency: {}", &result);
    }

    fn part2(&self) -> String {
        let values = &self
            .input
            .lines()
            .map(|value| value.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut unique_values = HashSet::new();
        let mut frequency = 0;

        loop {
            for value in values {
                frequency += value;
                if !unique_values.insert(frequency) {
                    return format!("First repeated frequency: {}", frequency);
                }
            }
        }
    }
}
