use std::fs::read_to_string;

struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.trim().split('-').collect();
        let start = parts[0].parse::<i64>().unwrap();
        let end = parts[1].parse::<i64>().unwrap();

        Range { start, end }
    }

    fn inclusive_includes(&self, value: i64) -> bool {
        value >= self.start && value <= self.end
    }
}

pub fn solve(input_file: &str) -> String {
    let contents = read_to_string(input_file).expect("Could not read file");

    let content_parts = contents.split("\n\n").collect::<Vec<&str>>();
    let range_lines = content_parts[0].lines().collect::<Vec<&str>>();
    let values_lines = content_parts[1].lines().collect::<Vec<&str>>();

    let ranges: Vec<Range> = range_lines.iter().map(|line| Range::new(line)).collect();

    let mut count_included = 0;
    for value_line in values_lines {
        let value = value_line.trim().parse::<i64>().unwrap();
        if ranges.iter().any(|range| range.inclusive_includes(value)) {
            count_included += 1;
        }
    }

    println!("Count of included values: {}", count_included);

    // Next, find the total number of unique integers included in any range
    // This will require finding and merging overlapping ranges first
    // And then just adding up the sizes of the merged ranges
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged_ranges: Vec<Range> = Vec::new();
    for range in sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.start <= last.end {
                // Overlapping ranges, merge them
                last.end = last.end.max(range.end);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    let total_unique_integers: i64 = merged_ranges
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum();

    format!(
        "Total unique integers in all ranges: {}",
        total_unique_integers
    )
}
