use crate::days::Day;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;


pub struct Day12 {
    input: String,
}

impl Day12 {
    pub fn new() -> Day12 {
        let input = fs::read_to_string("res/day12.txt").unwrap();
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
            .to_string();

        Day12 { input }
    }
}

impl Day for Day12 {
    fn part1(&self) -> String {
        let initial_generation = parse_input(&self.input);

        println!("POTS {:#?}", initial_generation.pots);

        println!("TRANSITIONS {:#?}", initial_generation.transitions);

        format!("{}", 0)
    }

    fn part2(&self) -> String {
        format!("{}", 0)
    }
}

#[derive(Debug)]
struct Generation {
    pots: HashMap<usize, Pot>,
    transitions: Vec<Transition>,
}

#[derive(Debug)]
enum Pot {
    Plant,
    Empty,
}

impl FromStr for Pot {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        match input {
            "#" => Ok(Pot::Plant),
            "." => Ok(Pot::Empty),
            _ => Err(Box::<dyn Error>::from("Unknown pot state")),
        }
    }
}

#[derive(Debug)]
struct Transition {
    state: Vec<Pot>,
    result: Pot,
}

impl FromStr for Transition {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        let parts = input.split(" => ").collect::<Vec<&str>>();

            let state = parts[0]
                .chars()
                .map(|ch| ch.to_string().parse::<Pot>())
                .collect::<Result<Vec<Pot>, Box<dyn Error>>>()?;

            let result = parts[1].parse::<Pot>()?;

            Ok(Transition { state, result })
    }
}

fn parse_input(input: &str) -> Generation {
    let mut lines = input.lines();
    let initial_state_prefix_length = "initial state: ".len();

    let pots = lines
        .next()
        .unwrap()
        .char_indices()
        .skip(initial_state_prefix_length)
        .map(|(i, ch)| {
            (
                i - initial_state_prefix_length,
                ch.to_string().parse::<Pot>().unwrap(),
            )
        })
        .collect::<HashMap<usize, Pot>>();

    let transitions = lines
        .skip(1) // skip blank line between initial state & transitions
        .map(|line| line.parse::<Transition>().unwrap())
        .collect::<Vec<Transition>>();

    Generation { pots, transitions }
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day12::new().part1().ends_with(""));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day12::new().part2().ends_with(""));
    }
}
