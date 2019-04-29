use regex::Regex;
use std::fs;
use crate::days::Day;

pub struct Day3 {
    input: String,
}

impl Day3 {
    pub fn new() -> Day3 {
        let input = fs::read_to_string("res/day3.txt").unwrap();

        Day3 { input }
    }
}

impl Day for Day3 {
    fn part1(&self) -> String {
        let splitter = Regex::new(r"[@ ,:x]+").unwrap();
        let lines = self.input.lines();
        let mut fabric_squares = vec![vec![0; 1000]; 1000];

        for line in lines {
            let vars = splitter.split(line)
                .skip(1)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let (x_coord, y_coord, x_size, y_size) = (vars[0], vars[1], vars[2], vars[3]);

            for y in y_coord..(y_coord + y_size) {
                for x in x_coord..(x_coord + x_size) {
                    fabric_squares[x][y] += 1;
                }
            }
        }

        let result = fabric_squares.iter()
            .flatten()
            .filter(|square| **square > 1)
            .count();

        return format!("Overlapping square inches: {}", result);
    }

    fn part2(&self) -> String {
        let splitter = Regex::new(r"[#@ ,:x]+").unwrap();
        let lines = self.input.lines();
        let mut fabric_squares = vec![vec![0; 1000]; 1000];
        let mut line_vars = Vec::<(usize, usize, usize, usize, usize)>::new();

        for line in lines {
            let vars = splitter.split(line)
                .skip(1)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let (id, x_coord, y_coord, x_size, y_size) = (vars[0], vars[1], vars[2], vars[3], vars[4]);
            line_vars.push((id, x_coord, y_coord, x_size, y_size));

            for y in y_coord..(y_coord + y_size) {
                for x in x_coord..(x_coord + x_size) {
                    fabric_squares[x][y] += 1;
                }
            }
        }

        'lineloop: for line_var in line_vars {
            let (id, x_coord, y_coord, x_size, y_size) = (line_var.0, line_var.1, line_var.2, line_var.3, line_var.4);

            for y in y_coord..(y_coord + y_size) {
                for x in x_coord..(x_coord + x_size) {
                    if fabric_squares[x][y] != 1 {
                        continue 'lineloop;
                    };
                }
            }

            return format!("Claim that does not overlap: {}", id);
        }

        panic!("No non-overlapping claim found!")
    }
}
