use aoc_2018::day_runner::DayRunner;
use std::io;

fn main() {
    let day_runner = DayRunner::new();

    loop {
        println!();
        println!("Which day should I run? (1-25 or q to quit)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim() == "q" {
            println!("Later!");
            break;
        }

        let day = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("That's... not an option");
                continue;
            }
        };

        if day < 1 || day > 25 {
            println!("That's... not an option");
            continue;
        }

        day_runner.run_day(day);
    }
}
