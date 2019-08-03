use crate::days::Day;
use std::char;
use std::collections::VecDeque;

pub struct Day14 {
    input: String,
}

impl Day14 {
    pub fn new() -> Day14 {
        let input = "607331".to_string();

        Day14 { input }
    }
}

impl Day for Day14 {
    fn part1(&self) -> String {
        let input = self.input.parse::<usize>().unwrap();
        let mut recipes = vec![3u32, 7u32];
        let mut elf_1_index = 0;
        let mut elf_2_index = 1;
        let mut result = Vec::new();

        while recipes.len() < input + 10 {
            let elf_1_recipe = *recipes.get(elf_1_index).unwrap();
            let elf_2_recipe = *recipes.get(elf_2_index).unwrap();

            (elf_1_recipe + elf_2_recipe)
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .for_each(|n| {
                    recipes.push(n);

                    if input < recipes.len() && recipes.len() <= input + 10 {
                        result.push(n);
                    }
                });

            elf_1_index = (elf_1_index + 1 + elf_1_recipe as usize) % recipes.len();
            elf_2_index = (elf_2_index + 1 + elf_2_recipe as usize) % recipes.len();
        }

        let result = result
            .iter()
            .map(|n| char::from_digit(*n, 10).unwrap())
            .collect::<String>();

        format!("10 recipes after {}: {}", self.input, result)
    }

    // this could use something like fixed-vec-deque instead? https://crates.io/crates/fixed-vec-deque
    // also optimise because it takes literal seconds to run :[
    fn part2(&self) -> String {
        let input = &self.input;
        let mut recipes = vec![3u32, 7u32];
        let mut elf_1_index = 0;
        let mut elf_2_index = 1;
        let mut latest_recipes_vec = VecDeque::with_capacity(input.len());

        while latest_recipes_vec.len() < latest_recipes_vec.capacity() {
            latest_recipes_vec.push_back(0);
        }

        'tick: loop {
            let elf_1_recipe = *recipes.get(elf_1_index).unwrap();
            let elf_2_recipe = *recipes.get(elf_2_index).unwrap();

            let new_recipe = (elf_1_recipe + elf_2_recipe).to_string();
            let new_recipe = new_recipe.chars().map(|c| c.to_digit(10).unwrap());

            // this was a for_each iterator but there was an issue with breaking early?
            // note to self to look at that issue again to better understand it
            for n in new_recipe {
                recipes.push(n);
                latest_recipes_vec.pop_front();
                latest_recipes_vec.insert(input.len() - 1, n);

                let mut latest_recipes = latest_recipes_vec
                    .iter()
                    .map(|n| char::from_digit(*n, 10).unwrap())
                    .collect::<String>();

                latest_recipes.truncate(input.len());

                if latest_recipes == *input {
                    break 'tick;
                }
            }

            elf_1_index = (elf_1_index + 1 + elf_1_recipe as usize) % recipes.len();
            elf_2_index = (elf_2_index + 1 + elf_2_recipe as usize) % recipes.len();
        }

        let result = recipes.len() - input.len();

        format!("Recipe count until {} found: {}", input, result)
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day14::new().part1().ends_with("8610321414"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day14::new().part2().ends_with("20258123"));
    }
}
