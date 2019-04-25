use std::collections::HashMap;

pub mod day1;

pub fn init_days() -> HashMap<u32, fn()> {
    let mut days = HashMap::<u32, fn()>::with_capacity(25);
    days.insert(1, day1::run);

    days
}

pub fn run_day(days_map: &HashMap<u32, fn()>, day: &u32) {
    match days_map.get(day) {
        Some(f) => {
            println!();
            println!("========== DAY {} ==========", day);
            f()
        },
        None => println!("I haven't got to that day yet!"),
    };
}