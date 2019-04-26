use std::collections::HashMap;
use std::fs;
use crate::days::Day;

pub struct Day1 {
    input: String
}

impl Day1 {
    pub fn new() -> Day1 {
        let input = fs::read_to_string("res/day1.txt").unwrap();

        Day1 { input }
    }
}

impl Day for Day1 {
    fn part1(&self) -> String {
        let result = &self.input.lines()
            .map(|value| value.parse::<i32>().unwrap())
            .sum::<i32>();

        return format!("Resulting frequency: {}", &result);
    }

    fn part2(&self) -> String {
        let values = &self.input.lines()
            .map(|value| value.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut unique_values = HashMap::new();
        let mut frequency = 0;

        loop {
            for value in values {
                frequency += value;
                let counter = unique_values.entry(frequency).or_insert(0);

                if *counter > 0 {
                    return format!("First repeated frequency: {}", &frequency);
                }

                *counter += 1;
            };
        }
    }
}
