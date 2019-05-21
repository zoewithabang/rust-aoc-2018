use crate::days::Day;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::str::Lines;

pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new() -> Day6 {
        let input = fs::read_to_string("res/day6.txt").unwrap();

        Day6 { input }
    }
}

impl Day for Day6 {
    /// Naive unoptimised implementations
    /// Finds closest to every possible coord individually
    /// For and iter versions made, iter is commented out
    /// 100 samples in estimated 543.25 s (5050 iterations)
    /// Iter: 100 samples in estimated 638.87 s (5050 iterations)
    fn part1(&self) -> String {
        let lines = self.input.lines();
        let (points, max_x, max_y) = get_point_data(lines);
        let mut point_totals = HashMap::new();

        for point in points.iter() {
            point_totals.insert(point.id, 0);
        }

        let point_totals = part1_for(points, max_x, max_y, point_totals);
        //let point_totals = part1_iter(points, max_x, max_y, point_totals);

        let largest = point_totals.values().max().unwrap();

        format!("Largest non-infinite: {}", largest)
    }

    fn part2(&self) -> String {
        let lines = self.input.lines();
        let (points, max_x, max_y) = get_point_data(lines);

        let matches = (0..=max_y)
            .flat_map(|y| {
                let points = &points;

                (0..=max_x)
                    .map(move |x| {
                        let coords = Coords { x, y };

                        points
                            .iter()
                            .map(move |point| point.get_distance_from(&coords))
                            .sum::<i32>()
                    })
                    .filter(|total_distance| *total_distance < 10_000)
            })
            .count();

        format!("Total less than 10,000 away from all points: {}", matches)
    }
}

#[derive(Debug, Clone)]
struct Point {
    id: usize,
    location: Coords,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl Point {
    fn get_distance_from(&self, coords: &Coords) -> i32 {
        i32::abs(self.location.x - coords.x) + i32::abs(self.location.y - coords.y)
    }
}

fn get_point_data(lines: Lines) -> (Vec<Point>, i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;

    let points = lines
        .enumerate()
        .map(|(id, line)| {
            let coords = line
                .split(", ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if coords[0] > max_x {
                max_x = coords[0];
            }

            if coords[1] > max_y {
                max_y = coords[1];
            }

            Point {
                id,
                location: Coords {
                    x: coords[0],
                    y: coords[1],
                },
            }
        })
        .collect::<Vec<Point>>();

    (points, max_x, max_y)
}

#[allow(dead_code)]
fn part1_for(
    points: Vec<Point>,
    max_x: i32,
    max_y: i32,
    mut point_totals: HashMap<usize, u32>,
) -> HashMap<usize, u32> {
    for y in 0..=max_y {
        for x in 0..=max_x {
            let coords = Coords { x, y };

            let mut distances = points
                .iter()
                .map(|point| {
                    let distance = point.get_distance_from(&coords);

                    (point.id, distance)
                })
                .collect::<Vec<(usize, i32)>>();

            distances.sort_by(|(_, distance1), (_, distance2)| distance1.cmp(distance2));

            // if closest two are not equal (if equal, closer to neither)
            if distances[0].1 != distances[1].1 {
                // if point is on the boundary, closest can go to infinity, remove
                // else increment closest cell count
                if x == 0 || y == 0 || x == max_x || y == max_y {
                    point_totals.remove(&distances[0].0);
                } else {
                    point_totals.entry(distances[0].0).and_modify(|x| *x += 1);
                }
            }
        }
    }

    point_totals
}

#[allow(dead_code)]
fn part1_iter(
    points: Vec<Point>,
    max_x: i32,
    max_y: i32,
    point_totals: HashMap<usize, u32>,
) -> HashMap<usize, u32> {
    (0..=max_y)
        .flat_map(|y| {
            let points = &points;

            (0..=max_x).map(move |x| {
                let coords = Coords { x, y };

                points
                    .iter()
                    .map(|point| {
                        let distance = point.get_distance_from(&coords);

                        (point.id, coords.clone(), distance)
                    })
                    .sorted_by(|(_, _, distance1), (_, _, distance2)| distance1.cmp(distance2))
                    .take(2)
                    .collect::<Vec<(usize, Coords, i32)>>()
            })
        })
        .fold(point_totals, |mut rolling_totals, closest_two| {
            {
                let distance1 = closest_two[0].2;
                let distance2 = closest_two[1].2;

                if distance1 != distance2 {
                    let x = closest_two[0].1.x;
                    let y = closest_two[0].1.y;
                    let id = closest_two[0].0;

                    // if point is on the boundary, closest can go to infinity, remove
                    // else increment closest cell count
                    if x == 0 || y == 0 || x == max_x || y == max_y {
                        rolling_totals.remove(&id);
                    } else {
                        rolling_totals.entry(id).and_modify(|x| *x += 1);
                    }
                }

                rolling_totals
            }
        })
}
