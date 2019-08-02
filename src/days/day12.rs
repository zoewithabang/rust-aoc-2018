use crate::days::Day;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

pub struct Day12 {
    input: String,
}

impl Day12 {
    pub fn new() -> Day12 {
        let input = fs::read_to_string("res/day12.txt").unwrap();

        Day12 { input }
    }
}

impl Day for Day12 {
    fn part1(&self) -> String {
        let mut generation = parse_input(&self.input).unwrap();

        (1..=20).for_each(|_| {
            generation = generation.next().unwrap();
        });

        let result = generation
            .pots
            .iter()
            .filter(|(_, pot)| **pot == Pot::Plant)
            .map(|(i, _)| i)
            .sum::<i32>();

        format!("Sum of pots which contain a plant: {}", result)
    }

    fn part2(&self) -> String {
        let mut generation = parse_input(&self.input).unwrap();
        let mut generation_count = 0;
        let mut latest_plant_count = 0;
        let mut latest_diff = 0;

        loop {
            generation_count += 1;
            generation = generation.next().unwrap();

            let plant_count = generation
                .pots
                .iter()
                .filter(|(_, pot)| **pot == Pot::Plant)
                .map(|(i, _)| i)
                .sum::<i32>();

            let difference = plant_count - latest_plant_count;
            latest_plant_count = plant_count;

            if difference == latest_diff {
                break;
            } else {
                latest_diff = difference;
            }
        }

        let remaining_gen_count = 50_000_000_000 - generation_count as i64;
        let result = latest_plant_count as i64 + (remaining_gen_count * latest_diff as i64);

        format!(
            "After 50 billion, sum of pots which contain a plant: {}",
            result
        )
    }
}

#[derive(Debug)]
struct Generation {
    number: i32,
    pots: BTreeMap<i32, Pot>,
    transitions: Vec<Transition>,
}

impl Generation {
    fn new(
        number: i32,
        pots: BTreeMap<i32, Pot>,
        transitions: Vec<Transition>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut generation = Generation {
            number,
            pots,
            transitions,
        };

        generation.pad_pots()?;

        Ok(generation)
    }

    fn next(&mut self) -> Result<Self, Box<dyn Error>> {
        let (first_pot_index, _) = self
            .pots
            .iter()
            .find(|(_, pot)| **pot == Pot::Plant)
            .ok_or("No plants found")?;

        let (last_pot_index, _) = self
            .pots
            .iter()
            .rev()
            .find(|(_, pot)| **pot == Pot::Plant)
            .ok_or("No plants found")?;

        let mut next_generation_pots = BTreeMap::new();

        (first_pot_index - 2..=*last_pot_index + 2).for_each(|i| {
            let mut pot_group = Vec::new();
            (i - 2..=i + 2).for_each(|j| pot_group.push(self.pots.get(&j)));
            let mut next_result = Pot::Empty;

            self.transitions.iter().for_each(|transition| {
                if transition
                    .state
                    .iter()
                    .zip(&pot_group)
                    .all(|(a, b)| b.is_some() && a == b.unwrap())
                {
                    next_result = transition.result.clone();

                    return;
                }
            });

            next_generation_pots.insert(i, next_result.clone());
        });

        self.number += 1;

        Ok(Generation::new(
            self.number + 1,
            next_generation_pots,
            self.transitions.clone(),
        )?)
    }

    fn pad_pots(&mut self) -> Result<(), Box<dyn Error>> {
        let first_pot_index = self
            .pots
            .iter()
            .find(|(_, pot)| **pot == Pot::Plant)
            .map(|(x, _)| *x)
            .ok_or("No plants found")?;

        let last_pot_index = self
            .pots
            .iter()
            .rev()
            .find(|(_, pot)| **pot == Pot::Plant)
            .map(|(x, _)| *x)
            .ok_or("No plants found")?;

        // if needed, specify 4 empty pots either side of known pots
        // 4 as transition rules specify 5 sequential states
        // allows "....#" at left and "#...." at right
        if self.pots.get(&(first_pot_index - 4)).is_none() {
            ((first_pot_index - 4)..first_pot_index).for_each(|i| {
                self.pots.insert(i, Pot::Empty);
            });
        }

        if self.pots.get(&(last_pot_index + 4)).is_none() {
            ((last_pot_index + 1)..=(last_pot_index + 4)).for_each(|i| {
                self.pots.insert(i, Pot::Empty);
            });
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
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
            _ => Err("Unknown pot state".into()),
        }
    }
}

#[derive(Clone, Debug)]
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

fn parse_input(input: &str) -> Result<Generation, Box<dyn Error>> {
    let mut lines = input.lines();
    let initial_state_prefix_length = "initial state: ".len();

    let pots = lines
        .next()
        .unwrap()
        .char_indices()
        .skip(initial_state_prefix_length)
        .map(|(i, ch)| {
            (
                (i - initial_state_prefix_length) as i32,
                ch.to_string().parse::<Pot>().unwrap(),
            )
        })
        .collect::<BTreeMap<i32, Pot>>();

    let transitions = lines
        .skip(1) // skip blank line between initial state & transitions
        .map(|line| line.parse::<Transition>().unwrap())
        .collect::<Vec<Transition>>();

    Generation::new(0, pots, transitions)
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day12::new().part1().ends_with("4110"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day12::new().part2().ends_with("2650000000466"));
    }
}
