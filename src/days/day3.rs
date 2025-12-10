use std::fs::read_to_string;

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<char>,
    max_to_use: usize,
}

impl BatteryBank {
    fn new(line: &str, max_to_use: usize) -> Self {
        let batteries = line.trim().chars().collect();
        BatteryBank {
            batteries,
            max_to_use,
        }
    }

    /*
    Finds the TWELVE largest batteries to turn on.

    The battery banks are a list of single-digit chars.
    The total charge is determined by evaluating 12 chars togather as a numerical valie.

    To do this, we essentially need to "build" the largest twelve-digit nummber possible.
    At a minimum, the starting point is the first twelve batteries.

    From there, we scan right, looking for larger batteries to replace the current largest batteries on the left.
    However, we have to be careful to maintain the order of the batteries.
     */
    fn largest_charge(&self) -> u64 {
        let mut cur_selected_battries: Vec<char> = self
            .batteries
            .iter()
            .take(self.max_to_use)
            .cloned()
            .collect();
        let mut cur_selected_indexes: Vec<usize> = (0..self.max_to_use).collect();
        let mut cur_charge: u64 = cur_selected_battries
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        for i in 1..self.batteries.len() {
            let candidate_battery = self.batteries[i];

            for j in 0..self.max_to_use {
                if candidate_battery > cur_selected_battries[j] && i > cur_selected_indexes[j] {
                    // We found a better battery to use at position j
                    // But we need to make sure we can still maintain order
                    // The candidate battery is at index i in the original list
                    // The battery we're replacing is at index of the j-th selected battery in the original list
                    let index_of_replaced_battery = cur_selected_indexes[j];

                    // We can replace if the index is greater than the index of the replaced battery
                    // AND there are enough batteries left to fill the remaining slots
                    let batteries_left = self.batteries.len() - i - 1;
                    let slots_needed = self.max_to_use - j - 1;

                    if i > index_of_replaced_battery && batteries_left >= slots_needed {
                        // We can replace it + all subsequent batteries
                        println!(
                            "Replacing battery {} at index {} with battery {} at index {}",
                            cur_selected_battries[j],
                            index_of_replaced_battery,
                            candidate_battery,
                            i
                        );
                        cur_selected_battries[j] = candidate_battery;
                        cur_selected_indexes[j] = i;

                        for k in (j + 1)..cur_selected_battries.len() {
                            // Fill in the rest with the next available batteries
                            let next_index = cur_selected_indexes[k - 1] + 1;
                            cur_selected_battries[k] = self.batteries[next_index];
                            cur_selected_indexes[k] = next_index;
                        }

                        // Recalculate charge
                        cur_charge = cur_selected_battries
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap();
                        break; // Move to the next candidate battery
                    }
                }
            }
        }

        println!("Selected batteries: {:?}", cur_selected_battries);

        cur_charge
    }
}

pub fn solve(input_file: &str) -> String {
    let max_to_use = 12;
    let contents = read_to_string(input_file).expect("Failed to read range file");
    let lines = contents.lines();

    let batteries: Vec<BatteryBank> = lines
        .map(|line| BatteryBank::new(line, max_to_use))
        .collect();

    let largest_charge_sum = batteries
        .iter()
        .map(|bank| bank.largest_charge())
        .sum::<u64>();

    format!("Sum of largest charges: {}", largest_charge_sum)
}
