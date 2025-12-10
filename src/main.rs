mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <day> <input_file>", args[0]);
        return;
    }

    let day = &args[1];
    let input_file = &args[2];

    match day.as_str() {
        "day1" => println!("{}", days::day1::solve(input_file)),
        "day2" => println!("{}", days::day2::solve(input_file)),
        "day3" => println!("{}", days::day3::solve(input_file)),
        "day4" => println!("{}", days::day4::solve(input_file)),
        "day5" => println!("{}", days::day5::solve(input_file)),
        _ => eprintln!("Unknown day: {}", day),
    }
}
