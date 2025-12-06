mod day1;
mod day2;
mod day3;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use std::env;

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
        "day3" => day3(&args[1..]),
        _ => eprintln!("Unknown day: {}", day),
    }
}
