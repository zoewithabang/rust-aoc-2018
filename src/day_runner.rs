use crate::days::{
    day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, day7::Day7, day8::Day8,
    day9::Day9, Day,
};
use std::collections::HashMap;

pub struct DayRunner {
    days: HashMap<u32, Box<Day>>,
}

impl DayRunner {
    pub fn new() -> DayRunner {
        let mut days = HashMap::<u32, Box<Day>>::with_capacity(25);
        days.insert(1, Box::new(Day1::new()));
        days.insert(2, Box::new(Day2::new()));
        days.insert(3, Box::new(Day3::new()));
        days.insert(4, Box::new(Day4::new()));
        days.insert(5, Box::new(Day5::new()));
        days.insert(6, Box::new(Day6::new()));
        days.insert(7, Box::new(Day7::new()));
        days.insert(8, Box::new(Day8::new()));
        days.insert(9, Box::new(Day9::new()));

        DayRunner { days }
    }

    pub fn run_day(&self, day: u32) {
        match self.days.get(&day) {
            Some(day_instance) => run_day_internal(day, &day_instance),
            None => println!("I haven't got to that day yet!"),
        };
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
