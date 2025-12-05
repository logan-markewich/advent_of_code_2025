mod day1;
mod day2;

use std::env;
use crate::day1::day1;
use crate::day2::day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <combination_file>", args[0]);
        return;
    }

    let day = &args[1];
    match day.as_str() {
        "day1" => day1(&args[1..]),
        "day2" => day2(&args[1..]),
        _ => eprintln!("Unknown day: {}", day),
    }
}
