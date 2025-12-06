use std::fs::read_to_string;

struct IDRange {
    min: u64,
    max: u64,
}

impl IDRange {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.trim().split('-').collect();
        let min = parts[0].parse::<u64>().unwrap();
        let max = parts[1].parse::<u64>().unwrap();
        IDRange { min, max }
    }

    /*
    Finds all IDs in the range that are invalid.
    An ID is invalid if an subsequence of digits is repeated.

    For example, 123456 is valid, but 123123 is invalid because "123" is repeated.
    121234 is invalid because "12" is repeated.
    101 is valid because "1" is not repeated consecutively.
    222222 is invalid because "2" is repeated.
     */
    fn get_invalid_ids(&self) -> Vec<u64> {
        let mut invalid_ids = Vec::new();
        for id in self.min..=self.max {
            let id_str = id.to_string();
            let len = id_str.len();

            // An ID is invalid if it's made of a sequence repeated two or more times
            // The entire number must be: pattern + pattern + ... (at least 2 repetitions)
            let mut is_invalid = false;

            // Try all possible pattern lengths from 1 to len/2
            for pattern_len in 1..=(len / 2) {
                // Check if the entire string can be made by repeating this pattern
                if len % pattern_len == 0 {
                    let repetitions = len / pattern_len;
                    if repetitions >= 2 {
                        let pattern = &id_str[0..pattern_len];
                        let mut is_repeating = true;

                        for i in 1..repetitions {
                            let start = i * pattern_len;
                            let end = start + pattern_len;
                            if &id_str[start..end] != pattern {
                                is_repeating = false;
                                break;
                            }
                        }

                        if is_repeating {
                            is_invalid = true;
                            break;
                        }
                    }
                }
            }

            if is_invalid {
                invalid_ids.push(id);
            }
        }
        invalid_ids
    }
}

pub fn day2(args: &[String]) {
    if args.len() != 2 {
        eprintln!("Usage: {} <id_range_file>", args[0]);
        return;
    }

    let filename = &args[1];
    let contents = read_to_string(filename).expect("Failed to read file");
    let split_contents = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .collect::<Vec<&str>>();
    let id_ranges: Vec<IDRange> = split_contents
        .iter()
        .map(|line| IDRange::new(line))
        .collect();
    let invalid_ids = id_ranges
        .iter()
        .flat_map(|id_range| id_range.get_invalid_ids())
        .collect::<Vec<u64>>();

    let sum_invalid_ids: u64 = invalid_ids.iter().sum();
    println!("Sum of invalid IDs: {}", sum_invalid_ids);
}
