use crate::days::Day;
use std::collections::VecDeque;
use std::fs;

pub struct Day8 {
    input: String,
}

impl Day8 {
    pub fn new() -> Day8 {
        let input = fs::read_to_string("res/day8.txt").unwrap();

        Day8 { input }
    }
}

impl Day for Day8 {
    fn part1(&self) -> String {
        let mut numbers = self
            .input
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let root = parse_node(&mut numbers);
        let sum = root.sum_metadata();

        format!("Sum of all metadata entries: {}", sum)
    }

    fn part2(&self) -> String {
        let mut numbers = self
            .input
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let root = parse_node(&mut numbers);
        let value = root.get_value();

        format!("Value of root node: {}", value)
    }
}

fn parse_node(numbers: &mut VecDeque<usize>) -> Node {
    let children_count = numbers.pop_front().unwrap();
    let metadata_count = numbers.pop_front().unwrap();
    let mut children = Vec::with_capacity(children_count);
    let mut metadata = Vec::with_capacity(metadata_count);

    (0..children_count).for_each(|_| children.push(parse_node(numbers)));
    (0..metadata_count).for_each(|_| metadata.push(numbers.pop_front().unwrap()));

    Node { children, metadata }
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self
                .children
                .iter()
                .map(|child| child.sum_metadata())
                .sum::<usize>()
    }

    fn get_value(&self) -> usize {
        if self.children.is_empty() {
            return self.sum_metadata();
        }

        self.metadata
            .iter()
            .map(|entry| match self.children.get(*entry - 1) {
                Some(node) => node.get_value(),
                None => 0,
            })
            .sum()
    }
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day8::new().part1().ends_with("42768"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day8::new().part2().ends_with("34348"));
    }
}
