use std::fs::read_to_string;

struct LockDial {
    cur_position: i16,
    max_position: i16,
    times_hit_zero: i16,
}

enum Direction {
    Left,
    Right,
}

struct LockCommand {
    direction: Direction,
    ticks: i16,
}

impl LockDial {
    fn new(max_position: i16) -> Self {
        LockDial {
            cur_position: 50,
            max_position,
            times_hit_zero: 0,
        }
    }

    fn turn(&mut self, command: &LockCommand) {
        match command.direction {
            Direction::Left => self.turn_left(command.ticks),
            Direction::Right => self.turn_right(command.ticks),
        }
    }

    fn turn_right(&mut self, ticks: i16) {
        let new_position = self.cur_position + ticks;

        // Count how many times we pass through 0
        // We pass through 0 every max_position units
        // But we need to check if we actually cross it
        if self.cur_position == 0 {
            // Starting at 0, we hit it again after max_position ticks
            self.times_hit_zero += ticks / self.max_position;
        } else {
            // Starting at some position > 0
            // We need (max_position - cur_position) to reach 0 the first time
            // Then max_position for each subsequent crossing
            let ticks_to_first_zero = self.max_position - self.cur_position;
            if ticks >= ticks_to_first_zero {
                // We reach 0 at least once
                let remaining_ticks = ticks - ticks_to_first_zero;
                self.times_hit_zero += 1 + (remaining_ticks / self.max_position);
            }
        }

        // Calculate final position
        self.cur_position = new_position % self.max_position;
    }

    fn turn_left(&mut self, ticks: i16) {
        if self.cur_position == 0 {
            // Starting at 0, we hit it again going backwards after max_position ticks
            self.times_hit_zero += ticks / self.max_position;
        } else {
            // We need cur_position ticks to reach 0 going backwards
            if ticks >= self.cur_position {
                // We reach 0 at least once
                let remaining_ticks = ticks - self.cur_position;
                self.times_hit_zero += 1 + (remaining_ticks / self.max_position);
            }
        }

        // Calculate final position
        let target = self.cur_position - ticks;
        self.cur_position = ((target % self.max_position) + self.max_position) % self.max_position;
    }
}

/*
Reads a lock combination from a file.

Each line in the file has a format like:
L68
L30
R48
*/
fn read_lock_combination(file_path: &str) -> Vec<LockCommand> {
    let contents = read_to_string(file_path).expect("Could not read file");

    contents
        .lines()
        .filter_map(|line| {
            let direction = match line.chars().next()? {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => return None,
            };
            let ticks: i16 = line[1..].parse().ok()?;

            Some(LockCommand { direction, ticks })
        })
        .collect()
}

pub fn solve(input_file: &str) -> String {
    let combination = read_lock_combination(input_file);
    let mut dial = LockDial::new(100);

    for command in &combination {
        dial.turn(command);
    }

    format!(
        "Final dial position: {}\nTimes hit zero position: {}",
        dial.cur_position, dial.times_hit_zero
    )
}
