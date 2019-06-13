use crate::days::Day;
use regex::Regex;
use std::fs;

pub struct Day10 {
    input: String,
}

impl Day10 {
    pub fn new() -> Day10 {
        let input = fs::read_to_string("res/day10.txt").unwrap();

        Day10 { input }
    }
}

impl Day for Day10 {
    fn part1(&self) -> String {
        let mut points = parse_input(&self.input);

        let (output, _) = find_message(&mut points);

        format!("Message in the sky:\n{}", output)
    }

    fn part2(&self) -> String {
        let mut points = parse_input(&self.input);

        let (_, seconds) = find_message(&mut points);

        format!("Seconds until message: {}", seconds)
    }
}

#[derive(Clone, Debug)]
struct Point {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

fn parse_input(input: &str) -> Vec<Point> {
    let matcher = Regex::new("-?\\d+").unwrap();

    input
        .lines()
        .map(|line| {
            let values = matcher
                .captures_iter(line)
                .map(|val| val.get(0).unwrap().as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            assert_eq!(values.len(), 4);

            Point {
                pos_x: values[0],
                pos_y: values[1],
                vel_x: values[2],
                vel_y: values[3],
            }
        })
        .collect::<Vec<Point>>()
}

fn find_message(mut points: &mut Vec<Point>) -> (String, u32) {
    let mut last_area_size = u64::max_value();
    let mut last_points = None;
    let mut seconds = 0;

    loop {
        let area_size = move_points_and_get_area(&mut points);

        if area_size > last_area_size {
            let output = get_point_output(&last_points.unwrap());

            return (output, seconds);
        }

        last_points = Some(points.clone());
        last_area_size = area_size;
        seconds += 1;
    }
}

fn get_point_output(points: &Vec<Point>) -> String {
    let (min_x, max_x, min_y, max_y) = get_range_of_points(points);
    let mut output = Vec::with_capacity(150);

    for y in min_y..=max_y {
        'coord: for x in min_x..=max_x {
            for point in points {
                if point.pos_x == x && point.pos_y == y {
                    output.push('█');

                    continue 'coord;
                }
            }

            output.push(' ');
        }

        output.push('\n');
    }

    output.iter().collect()
}

fn move_points_and_get_area(points: &mut Vec<Point>) -> u64 {
    points.iter_mut().for_each(|point| {
        point.pos_x += point.vel_x;
        point.pos_y += point.vel_y;
    });

    let (min_x, max_x, min_y, max_y) = get_range_of_points(points);

    ((max_x - min_x) as u64 * (max_y - min_y) as u64)
}

fn get_range_of_points(points: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();

    points.iter().for_each(|point| {
        if point.pos_x > max_x {
            max_x = point.pos_x;
        }

        if point.pos_y > max_y {
            max_y = point.pos_y;
        }

        if point.pos_x < min_x {
            min_x = point.pos_x;
        }

        if point.pos_y < min_y {
            min_y = point.pos_y;
        }
    });

    (min_x, max_x, min_y, max_y)
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        let incredible_string = "
█████   █       █████   █████    ████   ██████  █████   █████ 
█    █  █       █    █  █    █  █    █  █       █    █  █    █
█    █  █       █    █  █    █  █       █       █    █  █    █
█    █  █       █    █  █    █  █       █       █    █  █    █
█████   █       █████   █████   █       █████   █████   █████ 
█       █       █    █  █       █  ███  █       █  █    █  █  
█       █       █    █  █       █    █  █       █   █   █   █ 
█       █       █    █  █       █    █  █       █   █   █   █ 
█       █       █    █  █       █   ██  █       █    █  █    █
█       ██████  █████   █        ███ █  █       █    █  █    █
";

        assert!(Day10::new().part1().ends_with(incredible_string));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day10::new().part2().ends_with("10519"));
    }
}
