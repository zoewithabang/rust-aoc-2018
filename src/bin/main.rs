use aoc_2018::day_runner::DayRunner;
use clap::{App, Arg, ArgMatches};
use std::io;

fn main() {
    let args = parse_args();
    let day_runner = DayRunner::new();

    if let Some(day) = args.value_of("day") {
        let day = day.parse::<u32>().unwrap();

        if day < 1 || day > 25 {
            println!("That's... not an option");

            return;
        }

        if let Some(part) = args.value_of("part") {
            let part = part.parse::<u32>().unwrap();
            day_runner.run_day_part(day, part);
        } else {
            day_runner.run_day(day);
        }

        return;
    }

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

    fn parse_args<'a>() -> ArgMatches<'a> {
        App::new("aoc_2018")
            .usage("Run without args for CLI or specify day (& optional part) via options.")
            .arg(
                Arg::with_name("day")
                    .help("The day to run. Use part option to limit to single part.")
                    .takes_value(true)
                    .short("d")
                    .long("day"),
            )
            .arg(
                Arg::with_name("part")
                    .help("The part of the day to run.")
                    .requires("day")
                    .takes_value(true)
                    .short("p")
                    .long("part"),
            )
            .get_matches()
    }
}
