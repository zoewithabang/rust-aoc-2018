use crate::days::Day;
use std::cmp::Ordering;
use std::fs;

pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new() -> Day6 {
        let input = fs::read_to_string("res/day6.txt").unwrap();

        Day6 { input: input }
    }
}

impl Day for Day6 {
    fn part1(&self) -> String {
        let lines = self.input.lines();
        let mut max_x = 0;
        let mut max_y = 0;

        let points = lines
            .enumerate()
            .map(|(id, line)| {
                let coords = line
                    .split(", ")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                assert!(coords.len() == 2);

                if coords[0] > max_x {
                    max_x = coords[0];
                }

                if coords[1] > max_y {
                    max_y = coords[1];
                }

                Point {
                    id,
                    coords: Coords {
                        x: coords[0],
                        y: coords[1],
                    },
                }
            })
            .collect::<Vec<Point>>();

        for y in 0..max_y {
            for x in 0..max_x {
                let current_coords = Coords { x, y };
                let closest_distance = max_x * max_y;
                let closest_id = 0;

                for point in points.iter() {
                    //if point.
                }
            }
        }

        format!("Largest non-infinite: {:?}", points)
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug)]
struct Point {
    id: usize,
    coords: Coords,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Coords {
    x: u32,
    y: u32,
}
