use crate::days::Day;
use regex::Regex;
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::rc::Rc;

pub struct Day9 {
    input: String,
}

impl Day9 {
    pub fn new() -> Day9 {
        let input = fs::read_to_string("res/day9.txt").unwrap();

        Day9 { input }
    }
}

impl Day for Day9 {
    fn part1(&self) -> String {
        let (player_count, last_marble) = parse_input(&self.input).unwrap();

        let mut players = vec![RefCell::new(Player { score: 0 }); player_count];
        let mut circle = Circle::new(last_marble);

        play_game(&mut players, &mut circle);

        let result = players
            .iter()
            .map(|player| player.borrow().score)
            .max_by(|a, b| a.cmp(&b))
            .unwrap();

        format!("Winning elf's score: {}", result)
    }

    fn part2(&self) -> String {
        let (player_count, last_marble) = parse_input(&self.input).unwrap();
        let last_marble = last_marble * 100;

        let mut players = vec![RefCell::new(Player { score: 0 }); player_count];
        let mut circle = Circle::new(last_marble);

        play_game(&mut players, &mut circle);

        let result = players
            .iter()
            .map(|player| player.borrow().score)
            .max_by(|a, b| a.cmp(&b))
            .unwrap();

        format!("Winning elf's score: {}", result)
    }
}

#[derive(Clone, Debug)]
struct Player {
    score: u32,
}

#[derive(Debug)]
struct Marble {
    next: u32,
    previous: u32,
    value: u32,
}

#[derive(Debug)]
struct Circle {
    last_marble: u32,
    current_marble: Rc<RefCell<Marble>>,
    marbles: Vec<Rc<RefCell<Marble>>>,
}

impl Circle {
    fn new(last_marble: u32) -> Self {
        let initial_marble = Rc::new(RefCell::new(Marble {
            next: 0,
            previous: 0,
            value: 0,
        }));

        Circle {
            last_marble,
            current_marble: initial_marble.clone(),
            marbles: vec![initial_marble.clone()],
        }
    }

    fn take_turn(&mut self, player: &mut Player) {
        let (current_marble_value, current_marble_next_value) = self.get_current_marble_data();
        let next_value = self.marbles.len() as u32;

        if next_value % 23 == 0 {
            let (bonus, current_marble_value) = self.remove_bonus_marble(current_marble_value);
            self.current_marble = self.marbles[current_marble_value as usize].clone();

            let kept_marble = Rc::new(RefCell::new(Marble {
                next: next_value,
                previous: next_value,
                value: next_value,
            }));

            // pushed to list with no links to maintain array index = marble value
            self.marbles.push(kept_marble.clone());

            player.score += next_value + bonus;
        } else {
            let marble = self.place_next_marble(next_value, current_marble_next_value);

            self.current_marble = marble.clone();
            self.marbles.push(marble.clone());
        }
    }

    fn get_current_marble_data(&self) -> (u32, u32) {
        let latest_marble = self.current_marble.borrow();

        (latest_marble.value, latest_marble.next)
    }

    fn remove_bonus_marble(&mut self, latest_value: u32) -> (u32, u32) {
        let previous_steps = 7;
        let bonus_marble_position = (0..previous_steps).fold(latest_value, |value, _| {
            self.marbles[value as usize].borrow().previous
        });

        let mut bonus_marble = self.marbles[bonus_marble_position as usize].borrow_mut();
        let mut previous = self.marbles[bonus_marble.previous as usize].borrow_mut();
        let mut next = self.marbles[bonus_marble.next as usize].borrow_mut();

        previous.next = next.value;
        next.previous = previous.value;

        // not required, just dummying out the links for debug
        bonus_marble.next = bonus_marble.value;
        bonus_marble.previous = bonus_marble.value;

        (bonus_marble.value, next.value)
    }

    fn place_next_marble(&mut self, next_value: u32, latest_next: u32) -> Rc<RefCell<Marble>> {
        let mut new_previous_marble = self.marbles[latest_next as usize].borrow_mut();

        match self.marbles[new_previous_marble.next as usize].try_borrow_mut() {
            Ok(mut new_next_marble) => {
                new_previous_marble.next = next_value;
                new_next_marble.previous = next_value;

                Rc::new(RefCell::new(Marble {
                    next: new_next_marble.value,
                    previous: new_previous_marble.value,
                    value: next_value,
                }))
            }
            Err(_) => {
                //if try_borrow fails for next, previous and next are same
                //this should only happen on the first turn
                new_previous_marble.next = next_value;
                new_previous_marble.previous = next_value;

                Rc::new(RefCell::new(Marble {
                    next: new_previous_marble.value,
                    previous: new_previous_marble.value,
                    value: next_value,
                }))
            }
        }
    }
}

fn parse_input(input: &str) -> Result<(usize, u32), Box<Error>> {
    let matcher =
        Regex::new("(?P<players>\\d+) players; last marble is worth (?P<points>\\d+) points")
            .unwrap();

    let captures = matcher.captures(input).ok_or("No captures found")?;

    let player_count = captures["players"].parse()?;
    let final_score = captures["points"].parse()?;

    Ok((player_count, final_score))
}

fn play_game(players: &mut Vec<RefCell<Player>>, circle: &mut Circle) {
    let mut players_cycle = players.iter().cycle();

    (0..circle.last_marble).for_each(|_| {
        circle.take_turn(&mut players_cycle.next().unwrap().borrow_mut());
    });
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day9::new().part1().ends_with("424112"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day9::new().part2().ends_with("3487352628"));
    }
}
