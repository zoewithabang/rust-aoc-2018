#![allow(dead_code)] // alt implementations in some days
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub trait Day {
    fn run(&self) {
        self.part1();
        self.part2();
    }

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
