use std::fs::read_to_string;

struct PaperMap {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl PaperMap {
    fn new(lines: &[String]) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut grid = vec![vec![' '; width]; height];

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid[y][x] = ch;
            }
        }

        PaperMap { width, height, grid }
    }

    // Finds all coords with < max_adjacent '@' characters
    fn get_coords_with_max_adjacent(&mut self, max_adjacent: usize) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let current_char = self.grid[y][x];
                if current_char != '@' {
                    continue;
                }
                let mut adjacent_count = 0;

                // Check all 8 directions
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dy == 0 && dx == 0 {
                            continue; // Skip self
                        }

                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;

                        if ny >= 0 && ny < self.height as i32 && nx >= 0 && nx < self.width as i32 {
                            if self.grid[ny as usize][nx as usize] == '@' {
                                // println!("Found adjacent @ at ({}, {})", nx, ny);
                                adjacent_count += 1;
                            }
                        }
                    }
                }

                if adjacent_count < max_adjacent {
                    coords.push((x, y));
                }
            }
        }

        // Remove all marked coordinates from the grid
        for (x, y) in &coords {
            self.grid[*y][*x] = 'x';
        }

        coords
    }
}

pub fn day4(args: &[String]) {
    if args.len() != 2 {
        eprintln!("Usage: {} <combination_file>", args[0]);
        return;
    }

    let filename = &args[1];
    let contents = read_to_string(filename).expect("Could not read file");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let mut paper_map = PaperMap::new(&lines);
    let coords = paper_map.get_coords_with_max_adjacent(4);
    let mut coord_adjusted = coords.len();
    while !coords.is_empty() {
        let new_coords = paper_map.get_coords_with_max_adjacent(4);
        if new_coords.is_empty() {
            break;
        }
        coord_adjusted += new_coords.len();
    }

    println!("Found {} valid coordinates:", coord_adjusted);
}