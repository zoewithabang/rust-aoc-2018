use crate::days::Day;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;

type Step = char;

pub struct Day7 {
    input: String,
}

impl Day7 {
    pub fn new() -> Day7 {
        let input = fs::read_to_string("res/day7.txt").unwrap();

        Day7 { input }
    }
}

impl Day for Day7 {
    fn part1(&self) -> String {
        let (all_steps, mut instruction_map) = parse_instructions(&self.input);
        let all_length = all_steps.len();
        let mut completed_steps = Vec::with_capacity(all_length);

        while completed_steps.len() < all_length {
            for step in &all_steps {
                // skip if step is already completed or still has prerequisites
                if completed_steps.contains(step) || !instruction_map[&step].is_empty() {
                    continue;
                }

                completed_steps.push(*step);

                // remove completed step from prerequisites
                for prerequisites in instruction_map.values_mut() {
                    if let Some(index) = prerequisites
                        .iter()
                        .position(|prerequisite| *prerequisite == *step)
                    {
                        prerequisites.remove(index);
                    }
                }

                break;
            }
        }

        let order = completed_steps.iter().collect::<String>();

        format!("Order of steps: {}", order)
    }

    fn part2(&self) -> String {
        let (all_steps, mut instruction_map) = parse_instructions(&self.input);
        let all_length = all_steps.len();
        let mut in_progress_steps = Vec::new();
        let mut completed_steps = Vec::with_capacity(all_length);
        let mut worker_collection = WorkerCollection::new(5);
        let mut seconds = 0;

        while completed_steps.len() < all_length {
            worker_collection.run_second(
                &mut seconds,
                &mut instruction_map,
                &all_steps,
                &mut in_progress_steps,
                &mut completed_steps,
            );
        }

        format!("Seconds to complete steps: {}", seconds)
    }
}

struct WorkerCollection {
    workers: Vec<Worker>,
}

struct Worker {
    working_on: Option<Step>,
    busy_until: u32,
}

impl WorkerCollection {
    fn new(amount: u32) -> WorkerCollection {
        let workers = (0..amount)
            .map(|_| Worker {
                working_on: None,
                busy_until: 0,
            })
            .collect::<Vec<Worker>>();

        WorkerCollection { workers }
    }

    fn run_second(
        &mut self,
        seconds: &mut u32,
        instruction_map: &mut BTreeMap<Step, Vec<Step>>,
        all_steps: &Vec<Step>,
        in_progress_steps: &mut Vec<Step>,
        completed_steps: &mut Vec<Step>,
    ) {
        for worker in &mut self.workers {
            if worker.working_on != None && worker.busy_until > *seconds {
                continue;
            } else if let Some(finished_step) = worker.working_on {
                completed_steps.push(finished_step);

                let index = in_progress_steps
                    .iter()
                    .position(|step| *step == finished_step)
                    .unwrap();

                in_progress_steps.remove(index);

                // remove completed step from prerequisites
                for prerequisites in instruction_map.values_mut() {
                    if let Some(index) = prerequisites
                        .iter()
                        .position(|prerequisite| *prerequisite == finished_step)
                    {
                        prerequisites.remove(index);
                    }
                }

                worker.working_on = None;

                // if finished, no action since last increment, decrement and return
                if completed_steps.len() == all_steps.len() {
                    *seconds -= 1;

                    return;
                }
            }

            for step in all_steps {
                // skip if step is already completed, in progress or still has prerequisites
                if completed_steps.contains(step)
                    || in_progress_steps.contains(step)
                    || !instruction_map[&step].is_empty()
                {
                    continue;
                }

                worker.working_on = Some(*step);
                worker.busy_until = get_busy_until(*seconds, *step);

                in_progress_steps.push(*step);

                break;
            }
        }

        *seconds += 1;
    }
}

fn parse_instructions(input: &str) -> (Vec<Step>, BTreeMap<Step, Vec<Step>>) {
    let matcher = Regex::new("Step (.) must be finished before step (.) can begin.").unwrap();
    let mut instruction_map = BTreeMap::new();

    matcher.captures_iter(input).for_each(|capture| {
        let prerequisite = capture[1].chars().next().unwrap();
        let step = capture[2].chars().next().unwrap();

        let entry = instruction_map.entry(step).or_insert_with(Vec::new);

        entry.push(prerequisite);

        instruction_map.entry(prerequisite).or_insert_with(Vec::new);
    });

    let all_steps = instruction_map.keys().cloned().collect::<Vec<char>>();

    (all_steps, instruction_map)
}

fn get_busy_until(initial_time: u32, step: Step) -> u32 {
    // initial time is the start time
    // "(step as u32) - b'A' as u32 + 1" maps A to 1, B to 2, etc
    // 60 addition specified in the day challenge
    initial_time + (step as u32) - b'A' as u32 + 1 + 60
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day7::new().part1().ends_with("HEGMPOAWBFCDITVXYZRKUQNSLJ"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day7::new().part2().ends_with("1226"));
    }
}
