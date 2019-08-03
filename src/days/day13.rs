use crate::days::Day;
use std::cmp::Ordering;
use std::fs;

pub struct Day13 {
    input: String,
}

impl Day13 {
    pub fn new() -> Day13 {
        let input = fs::read_to_string("res/day13.txt").unwrap();

        Day13 { input }
    }
}

impl Day for Day13 {
    fn part1(&self) -> String {
        let mut network = parse_input(&self.input);

        while network.crashed_carts.len() == 0 {
            network.tick();
        }

        let cart = network
            .carts
            .iter()
            .filter(|cart| network.crashed_carts.contains(&cart.id))
            .last()
            .unwrap();

        format!("Location of first crash: {},{}", cart.x, cart.y)
    }

    fn part2(&self) -> String {
        let mut network = parse_input(&self.input);
        let expected_crash_count = network.carts.len() - 1;

        while network.crashed_carts.len() != expected_crash_count {
            network.tick();
        }

        let cart = network
            .carts
            .iter()
            .filter(|cart| !network.crashed_carts.contains(&cart.id))
            .next()
            .unwrap();

        format!("Location of last cart: {},{}", cart.x, cart.y)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cart {
    id: u32,
    x: usize,
    y: usize,
    facing: Direction,
    next_crossing: CrossingOption,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CrossingOption {
    Left,
    Forward,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Location {
    Horizontal,
    Vertical,
    SlashCorner,
    BackslashCorner,
    Crossing,
    Blank,
}

struct Network {
    locations: Vec<Vec<Location>>,
    carts: Vec<Cart>,
    crashed_carts: Vec<u32>,
    next_cart_id: u32,
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CrossingOption {
    fn next(&self) -> Self {
        match self {
            CrossingOption::Left => CrossingOption::Forward,
            CrossingOption::Forward => CrossingOption::Right,
            CrossingOption::Right => CrossingOption::Left,
        }
    }
}

impl Direction {
    fn next_after_crossing(&self, crossing: &CrossingOption) -> Self {
        match crossing {
            CrossingOption::Left if *self == Direction::North => Direction::West,
            CrossingOption::Left if *self == Direction::East => Direction::North,
            CrossingOption::Left if *self == Direction::South => Direction::East,
            CrossingOption::Left if *self == Direction::West => Direction::South,
            CrossingOption::Forward => self.clone(),
            CrossingOption::Right if *self == Direction::North => Direction::East,
            CrossingOption::Right if *self == Direction::East => Direction::South,
            CrossingOption::Right if *self == Direction::South => Direction::West,
            CrossingOption::Right if *self == Direction::West => Direction::North,
            _ => unreachable!(),
        }
    }
}

impl Network {
    fn new() -> Self {
        Network {
            locations: Vec::new(),
            carts: Vec::new(),
            crashed_carts: Vec::new(),
            next_cart_id: 0,
        }
    }

    fn add_cart(&mut self, x: usize, y: usize, facing: Direction) {
        self.carts.push(Cart {
            id: self.next_cart_id,
            x,
            y,
            facing,
            next_crossing: CrossingOption::Left,
        });

        self.next_cart_id += 1;
    }

    fn tick(&mut self) {
        // sort to iterate over top left cart to bottom right cart
        self.carts.sort();
        let mut previous_carts = self.carts.clone();

        for cart in &mut self.carts {
            // skip if this cart has already crashed
            if self.crashed_carts.contains(&cart.id) {
                continue;
            }

            let crashed_carts = self.crashed_carts.clone();

            let carts_on_space = previous_carts
                .iter()
                .filter(|cart| !crashed_carts.contains(&cart.id))
                .filter(|c| c.x == cart.x && c.y == cart.y);

            if carts_on_space.clone().count() > 1 {
                for c in carts_on_space {
                    self.crashed_carts.push(c.id);
                }
            }

            // skip if this cart has just been found to have crashed
            if self.crashed_carts.contains(&cart.id) {
                continue;
            }

            match cart.facing {
                Direction::North => cart.y -= 1,
                Direction::East => cart.x += 1,
                Direction::South => cart.y += 1,
                Direction::West => cart.x -= 1,
            }

            let location = &self.locations[cart.y][cart.x];

            match location {
                Location::SlashCorner => match cart.facing {
                    Direction::North => cart.facing = Direction::East,
                    Direction::East => cart.facing = Direction::North,
                    Direction::South => cart.facing = Direction::West,
                    Direction::West => cart.facing = Direction::South,
                },
                Location::BackslashCorner => match cart.facing {
                    Direction::North => cart.facing = Direction::West,
                    Direction::East => cart.facing = Direction::South,
                    Direction::South => cart.facing = Direction::East,
                    Direction::West => cart.facing = Direction::North,
                },
                Location::Crossing => {
                    cart.facing = cart.facing.next_after_crossing(&cart.next_crossing);
                    cart.next_crossing = cart.next_crossing.next();
                }
                Location::Blank => unreachable!(),
                _ => (), // horizontal and vertical do not cause direction changes
            }

            previous_carts
                .iter_mut()
                .filter(|c| cart.id == c.id)
                .for_each(|c| {
                    c.x = cart.x;
                    c.y = cart.y;
                });
        }
    }
}

fn parse_input(input: &str) -> Network {
    let mut network = Network::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let line = line
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                '-' => Location::Horizontal,
                '|' => Location::Vertical,
                '/' => Location::SlashCorner,
                '\\' => Location::BackslashCorner,
                '+' => Location::Crossing,
                '^' => {
                    network.add_cart(x, y, Direction::North);

                    Location::Vertical
                }
                'v' => {
                    network.add_cart(x, y, Direction::South);

                    Location::Vertical
                }
                '<' => {
                    network.add_cart(x, y, Direction::West);

                    Location::Horizontal
                }
                '>' => {
                    network.add_cart(x, y, Direction::East);

                    Location::Horizontal
                }
                _ => Location::Blank,
            })
            .collect::<Vec<_>>();

        network.locations.push(line);
    });

    network
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day13::new().part1().ends_with("38,72"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day13::new().part2().ends_with("68,27"));
    }
}
