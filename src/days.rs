pub mod day1;
pub mod day2;

pub trait Day {
    fn run(&self) {
        &self.part1();
        &self.part2();
    }

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

