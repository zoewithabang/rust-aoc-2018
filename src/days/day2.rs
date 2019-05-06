use crate::days::Day;
use std::collections::HashMap;
use std::fs;

pub struct Day2 {
    input: String,
}

impl Day2 {
    pub fn new() -> Day2 {
        let input = fs::read_to_string("res/day2.txt").unwrap();

        Day2 { input }
    }
}

impl Day for Day2 {
    fn part1(&self) -> String {
        let mut two_count = 0;
        let mut three_count = 0;
        let lines = self.input.lines();

        for line in lines {
            let mut map = HashMap::new();

            line.chars().for_each(|x| {
                let count = map.entry(x).or_insert(0);
                *count += 1;
            });

            if map.values().any(|x| *x == 2) {
                two_count += 1;
            }

            if map.values().any(|x| *x == 3) {
                three_count += 1;
            }
        }

        let result = two_count * three_count;

        return format!("Checksum: {}", &result);
    }

    fn part2(&self) -> String {
        let lines = self.input.lines().collect::<Vec<&str>>();
        let line_count = lines.len();
        let line_size = lines[0].len();

        for i in 0..line_count {
            for j in (i + 1)..line_count {
                let (common, _) = lines[i]
                    .chars()
                    .zip(lines[j].chars())
                    .filter(|&(a, b)| a == b)
                    .unzip::<char, char, Vec<_>, Vec<_>>();

                if common.len() == line_size - 1 {
                    return format!(
                        "Correct boxes common letters: {}",
                        &common.iter().collect::<String>()
                    );
                }
            }
        }

        panic!("Correct boxes were not found!");
    }
}
