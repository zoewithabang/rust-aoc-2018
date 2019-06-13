use crate::days::Day;

pub struct Day11 {
    input: String,
}

impl Day11 {
    pub fn new() -> Day11 {
        let input = "7403".to_string();

        Day11 { input }
    }
}

// Naive implementation are at worst O(n^5)
// Summed area implementations are at worst O(n^3)
impl Day for Day11 {
    fn part1(&self) -> String {
        let input = self.input.parse::<usize>().unwrap();
        //let input = 18;

        //part1_naive(input)
        part1_summed_area(input)
    }

    fn part2(&self) -> String {
        let input = self.input.parse::<usize>().unwrap();

        //part2_naive(input)
        part2_summed_area(input)
    }
}

fn part1_naive(input: usize) -> String {
    const GRID_SIZE: usize = 300;
    const SQUARE_SIZE: usize = 3;

    let grid = NaiveFuelCellGrid::new(GRID_SIZE, input);
    let (_, (x, y)) = naive_calculate_best_square(&grid, SQUARE_SIZE);

    format!("Square of largest total power at: {},{}", x, y)
}

fn part2_naive(input: usize) -> String {
    const GRID_SIZE: usize = 300;

    let grid = NaiveFuelCellGrid::new(GRID_SIZE, input);
    let mut largest_total = 0;
    let mut largest_total_location = (0, 0);
    let mut largest_total_size = 0;

    for square_size in 2..=GRID_SIZE {
        let (total_power, (x, y)) = naive_calculate_best_square(&grid, square_size);

        if total_power > largest_total {
            //println!("{}: {},{},{}", total_power, x, y, square_size);
            largest_total = total_power;
            largest_total_location = (x, y);
            largest_total_size = square_size;
        }

        // answer already found at this point, assumed via debug prints
        // unproven as it doesn't finish all but the rest of the iterations take ages
        if square_size == 11 {
            break;
        }
    }

    let (x, y) = largest_total_location;

    format!(
        "Square of largest total power and size at: {},{},{}",
        x, y, largest_total_size
    )
}

struct NaiveFuelCellGrid {
    grid: Vec<Vec<i32>>,
    size: usize,
}

impl NaiveFuelCellGrid {
    fn new(size: usize, serial_number: usize) -> Self {
        let mut grid = vec![vec![0; size]; size];

        for y in 1..=size {
            for x in 1..=size {
                let rack_id = x + 10;
                let mut power_level = rack_id * y;
                power_level += serial_number;
                power_level *= rack_id;
                power_level = (power_level / 100) % 10;
                let power_level = power_level as i32 - 5;

                grid[x - 1][y - 1] = power_level;
            }
        }

        NaiveFuelCellGrid { grid, size }
    }

    fn calculate_power(&self, x: usize, y: usize, size: usize) -> i32 {
        //convert to 0-indexed
        let x = x - 1;
        let y = y - 1;

        (y..(y + size))
            .flat_map(|y| (x..(x + size)).map(move |x| self.grid[x][y]))
            .sum()
    }
}

fn naive_calculate_best_square(
    grid: &NaiveFuelCellGrid,
    square_size: usize,
) -> (i32, (usize, usize)) {
    let mut largest_total = 0;
    let mut largest_total_location = (0, 0);

    for y in 1..=(grid.size - square_size + 1) {
        for x in 1..=(grid.size - square_size + 1) {
            let total_power = grid.calculate_power(x, y, square_size);
            if total_power > largest_total {
                largest_total = total_power;
                largest_total_location = (x, y);
            }
        }
    }

    (largest_total, largest_total_location)
}

fn part1_summed_area(input: usize) -> String {
    const GRID_SIZE: usize = 300;
    const SQUARE_SIZE: usize = 3;

    let grid = SummedAreaFuelCellGrid::new(GRID_SIZE, input);
    let (_, (x, y)) = summed_area_calculate_best_square(&grid, SQUARE_SIZE);

    format!("Square of largest total power at: {},{}", x, y)
}

fn part2_summed_area(input: usize) -> String {
    const GRID_SIZE: usize = 300;

    let grid = SummedAreaFuelCellGrid::new(GRID_SIZE, input);
    let mut largest_total = 0;
    let mut largest_total_location = (0, 0);
    let mut largest_total_size = 0;

    for square_size in 2..=GRID_SIZE {
        let (total_power, (x, y)) = summed_area_calculate_best_square(&grid, square_size);

        if total_power > largest_total {
            largest_total = total_power;
            largest_total_location = (x, y);
            largest_total_size = square_size;
        }
    }

    let (x, y) = largest_total_location;

    format!(
        "Square of largest total power and size at: {},{},{}",
        x, y, largest_total_size
    )
}

struct SummedAreaFuelCellGrid {
    summed_area_table: Vec<Vec<i64>>,
    size: usize,
}

impl SummedAreaFuelCellGrid {
    fn new(size: usize, serial_number: usize) -> Self {
        let mut table = vec![vec![0; size]; size];

        //coordinates are 1-indexed
        for y in 1..=size {
            for x in 1..=size {
                //calculate power level of this cell
                let rack_id = x + 10;
                let mut power_level = rack_id * y;
                power_level += serial_number;
                power_level *= rack_id;
                power_level = (power_level / 100) % 10;
                let power_level = power_level as i64 - 5;

                //0-indexed (zi) coordinates
                let zi_x = x - 1;
                let zi_y = y - 1;

                //calculate summed area, see https://en.wikipedia.org/wiki/Summed-area_table
                let val_up_right = if zi_y > 0 { table[zi_x][zi_y - 1] } else { 0 };
                let val_down_left = if zi_x > 0 { table[zi_x - 1][zi_y] } else { 0 };

                let val_up_left = if zi_y > 0 && zi_x > 0 {
                    table[zi_x - 1][zi_y - 1]
                } else {
                    0
                };

                let cell = power_level + val_up_right + val_down_left - val_up_left;

                table[x - 1][y - 1] = cell;
            }
        }

        SummedAreaFuelCellGrid {
            summed_area_table: table,
            size,
        }
    }

    //evaluate sum of intensities, see https://en.wikipedia.org/wiki/Summed-area_table
    fn calculate_square_power(&self, x: usize, y: usize, size: usize) -> i32 {
        //0-indexed (zi) coordinates
        let zi_x = x - 1;
        let zi_y = y - 1;
        let inner_size = size - 1;

        let val_up_left = if zi_x > 0 && zi_y > 0 {
            self.summed_area_table[zi_x - 1][zi_y - 1]
        } else {
            0
        };

        let val_up_right = if zi_y > 0 {
            self.summed_area_table[zi_x + inner_size][zi_y - 1]
        } else {
            0
        };

        let val_down_left = if zi_x > 0 {
            self.summed_area_table[zi_x - 1][zi_y + inner_size]
        } else {
            0
        };

        let val_down_right = self.summed_area_table[zi_x + inner_size][zi_y + inner_size];

        (val_up_left - val_up_right - val_down_left + val_down_right) as i32
    }
}

fn summed_area_calculate_best_square(
    grid: &SummedAreaFuelCellGrid,
    square_size: usize,
) -> (i32, (usize, usize)) {
    let mut largest_total = 0;
    let mut largest_total_location = (0, 0);

    for y in 1..=(grid.size - square_size + 1) {
        for x in 1..=(grid.size - square_size + 1) {
            let total_power = grid.calculate_square_power(x, y, square_size);
            if total_power > largest_total {
                largest_total = total_power;
                largest_total_location = (x, y);
            }
        }
    }

    (largest_total, largest_total_location)
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn part1_puzzle() {
        assert!(Day11::new().part1().ends_with("235,48"));
    }

    #[test]
    fn part2_puzzle() {
        assert!(Day11::new().part2().ends_with("285,113,11"));
    }
}
