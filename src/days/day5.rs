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
        let length = react_polymer(&self.input);

        format!("Remaining units: {}", length)
    }

    fn part2(&self) -> String {
        let shortest_length = (b'A'..=b'Z')
            .map(|i| {
                let capital = i as char;
                let lower = capital.to_ascii_lowercase();

                let input = &self.input.replace(capital, "").replace(lower, "");

                react_polymer(input)
            })
            .min()
            .unwrap();

        format!("Shortest polymer: {}", shortest_length)
    }
}

/// Reacts the polymer, removing all neighbouring letters
/// that have different capitalisations and returns length
/// e.g. "aBbccdDC" -> "accdDC" -> "accC" -> "ac" -> 2
fn react_polymer(polymer: &str) -> usize {
    let start_polymer = polymer.chars().collect::<Vec<char>>();
    let mut end_polymer = Vec::<char>::new();

    for polymer in start_polymer {
        if let Some(latest_next) = end_polymer.last() {
            if polymer != *latest_next
                && polymer.to_ascii_lowercase() == latest_next.to_ascii_lowercase()
            {
                end_polymer.pop();
            } else {
                end_polymer.push(polymer);
            }
        } else {
            end_polymer.push(polymer);
        }
    }

    end_polymer.len()
}
