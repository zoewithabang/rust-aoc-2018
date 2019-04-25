use std::fs;
use crate::util::printer;
use std::collections::HashMap;

pub fn run() {
    let input = fs::read_to_string("res/day1.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    printer::print_part(1);

    let result = input.lines()
        .map(|value| value.parse::<i32>().unwrap())
        .sum::<i32>();

    println!("Resulting frequency: {}", result);
}

fn part2(input: &str) {
    printer::print_part(2);

    let values = input.lines()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut unique_values = HashMap::new();
    let mut frequency = 0;

    loop {
        for value in &values {
            frequency += value;
            let counter = unique_values.entry(frequency).or_insert(0);

            if *counter > 0 {
                println!("First repeated frequency: {}", frequency);
                return;
            }

            *counter += 1;
        };
    }
}