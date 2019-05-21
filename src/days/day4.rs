use crate::days::Day;
use chrono::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::Lines;

type GuardId = u32;

pub struct Day4 {
    input: String,
}

impl Day4 {
    pub fn new() -> Day4 {
        let input = fs::read_to_string("res/day4.txt").unwrap();

        Day4 { input }
    }
}

impl Day for Day4 {
    fn part1(&self) -> String {
        let lines = self.input.lines();
        let records = get_sorted_records_vec(&lines);
        let guard_sleeping_minutes = get_guard_sleeping_minutes(records);

        let (sleepiest_guard_id, minutes_slept) = guard_sleeping_minutes
            .iter()
            .map(|(guard_id, minute_maps)| (*guard_id, minute_maps.values().sum::<u32>()))
            .max_by(|(_, minutes_slept), (_, minutes_slept2)| minutes_slept.cmp(minutes_slept2))
            .unwrap();

        let (sleepiest_minute, sleep_count) = guard_sleeping_minutes[&sleepiest_guard_id]
            .iter()
            .max_by(|(_, sleep_count), (_, sleep_count2)| sleep_count.cmp(sleep_count2))
            .unwrap();

        format!("Slept most total guard ID: {} ({} minutes total), sleepiest minute: {} ({} times), multiplied: {}",
            sleepiest_guard_id,
            minutes_slept,
            sleepiest_minute,
            sleep_count,
            sleepiest_guard_id * sleepiest_minute)
    }

    fn part2(&self) -> String {
        let lines = self.input.lines();
        let records = get_sorted_records_vec(&lines);
        let guard_sleeping_minutes = get_guard_sleeping_minutes(records);

        let (sleepiest_guard_id, (sleepiest_minute, sleep_count)) = guard_sleeping_minutes
            .iter()
            .map(|(guard_id, minute_maps)| {
                (
                    *guard_id,
                    minute_maps
                        .iter()
                        .max_by(|(_, sleep_count), (_, sleep_count2)| sleep_count.cmp(sleep_count2))
                        .unwrap(),
                )
            })
            .max_by(|(_, (_, sleep_count)), (_, (_, sleep_count2))| sleep_count.cmp(sleep_count2))
            .unwrap();

        format!("Slept most on same minute guard ID: {}, sleepiest minute: {} ({} times), multiplied: {}",
            sleepiest_guard_id,
            sleepiest_minute,
            sleep_count,
            sleepiest_guard_id * sleepiest_minute)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    Start,
    Sleep,
    Wake,
}

#[derive(Debug, Eq, PartialEq)]
struct Record {
    timestamp: DateTime<Utc>,
    guard_id: Option<GuardId>,
    state: State,
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_sorted_records_vec(lines: &Lines) -> Vec<Record> {
    let id_finder = Regex::new(r"\d+").unwrap();
    let mut records = Vec::<Record>::with_capacity(1056); //day4.txt size

    for line in lines.clone() {
        let line_halves = line.split("] ").collect::<Vec<&str>>();
        let (timestamp, state) = (line_halves[0], line_halves[1]);
        let timestamp = Utc.datetime_from_str(timestamp, "[%Y-%m-%d %H:%M").unwrap();

        let record = match &state[..5] {
            "falls" => Record {
                timestamp,
                guard_id: None,
                state: State::Sleep,
            },
            "wakes" => Record {
                timestamp,
                guard_id: None,
                state: State::Wake,
            },
            "Guard" => {
                let guard_id = id_finder.captures(state).unwrap()[0]
                    .parse::<u32>()
                    .unwrap();
                Record {
                    timestamp,
                    guard_id: Some(guard_id),
                    state: State::Start,
                }
            }
            _ => panic!("Invalid state: {}", state),
        };

        records.push(record);
    }

    records.sort_unstable();

    records
}

fn get_guard_sleeping_minutes(records: Vec<Record>) -> HashMap<GuardId, HashMap<u32, u32>> {
    let mut current_guard = None;
    let mut slept_at_min = 0;
    let mut guard_sleeping_minutes = HashMap::<GuardId, HashMap<u32, u32>>::new();

    for record in records {
        match record.state {
            State::Start => {
                current_guard = record.guard_id;
            }
            State::Sleep => {
                slept_at_min = record.timestamp.minute();
            }
            State::Wake => {
                let sleeping_minutes = guard_sleeping_minutes
                    .entry(current_guard.unwrap())
                    .or_insert_with(HashMap::new);

                for minute in slept_at_min..record.timestamp.minute() {
                    let minute_entry = sleeping_minutes.entry(minute).or_insert(0);
                    *minute_entry += 1;
                }
            }
        }
    }

    guard_sleeping_minutes
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day4::new().part1().ends_with("12169"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day4::new().part2().ends_with("16164"));
    }
}
