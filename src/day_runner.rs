use crate::days::{
    day1::Day1, day10::Day10, day11::Day11, day12::Day12, day13::Day13, day14::Day14, day15::Day15,
    day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, day7::Day7, day8::Day8, day9::Day9,
    Day,
};
use std::collections::HashMap;

pub struct DayRunner {
    days: HashMap<u32, Box<dyn Day>>,
}

impl DayRunner {
    pub fn new() -> DayRunner {
        let mut days = HashMap::<u32, Box<dyn Day>>::with_capacity(25);
        days.insert(1, Box::new(Day1::new()));
        days.insert(2, Box::new(Day2::new()));
        days.insert(3, Box::new(Day3::new()));
        days.insert(4, Box::new(Day4::new()));
        days.insert(5, Box::new(Day5::new()));
        days.insert(6, Box::new(Day6::new()));
        days.insert(7, Box::new(Day7::new()));
        days.insert(8, Box::new(Day8::new()));
        days.insert(9, Box::new(Day9::new()));
        days.insert(10, Box::new(Day10::new()));
        days.insert(11, Box::new(Day11::new()));
        days.insert(12, Box::new(Day12::new()));
        days.insert(13, Box::new(Day13::new()));
        days.insert(14, Box::new(Day14::new()));
        days.insert(15, Box::new(Day15::new()));

        DayRunner { days }
    }

    pub fn run_day(&self, day: u32) {
        match self.days.get(&day) {
            Some(day_instance) => run_day_internal(day, &day_instance),
            None => println!("I haven't got to that day yet!"),
        };
    }

    pub fn run_day_part(&self, day: u32, part: u32) {
        match self.days.get(&day) {
            Some(day_instance) => run_day_part_internal(day, part, &day_instance),
            None => println!("I haven't got to that day yet!"),
        }
    }
}

fn run_day_internal(day_number: u32, day: &Box<dyn Day>) {
    println!();
    println!("========== DAY {} ==========", day_number);
    println!("===== Part 1 =====");
    println!("{}", day.part1());
    println!("===== Part 2 =====");
    println!("{}", day.part2());
}

fn run_day_part_internal(day_number: u32, day_part: u32, day: &Box<dyn Day>) {
    let part: Box<Fn() -> String> = match day_part {
        1 => Box::new(|| day.part1()),
        2 => Box::new(|| day.part2()),
        _ => Box::new(|| format!("There is no part {}!", day_part)),
    };

    println!();
    println!("========== DAY {} ==========", day_number);
    println!("===== Part {} =====", day_part);
    println!("{}", part());
}
