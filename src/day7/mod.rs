use std::str::Lines;

pub(crate) fn solve_day7() {
    // Get the input: 2D graph tracking the trachyon beams
    let binding = include_str!("day7-input.txt");
    // let binding = include_str!("day7-test.txt");
    let lines: Lines<'_> = binding.lines();

    let mut grid: Vec<Vec<char>> = lines.map(|s| s.trim().chars().collect()).collect();

    let splits = track_classical_trachron(&mut grid);

    println!("The trachyon beam splits {} times", splits);

    solve_quantum_trachyon(&grid);
}

/// Returns the number of possible timelines a quantum trachyon particle could have taken
fn solve_quantum_trachyon(grid: &Vec<Vec<char>>) -> usize {
    // Work down the grid, at each place counting the number of possible ways to get to that spot
    // Return the sum of the last numbers in every row

    // Initialize timelines to 2D array of 0s
    let mut timelines: Vec<Vec<usize>> = Vec::with_capacity(grid.len());
    for r in 0..grid.len() {
        timelines.push(vec![0; grid[r].len()]);
    }

    let mut row_count: usize = 0;

    for (r, filled_row) in grid.iter().enumerate() {
        // Work down one row at a time from the filled in map

        row_count = 0;
        for c in 0..filled_row.len() {
            // Count the number of ways a beam could have gotten to a certain spot
            // 'S': Base case, 1 start
            if grid[r][c] == 'S' {
                timelines[r][c] = 1;
            } else if grid[r][c] == '|' {
                // Add the total ways a beam could have gotten to that spot
                // Number above it + numbers above the splitters on each side
                let mut count = timelines[r - 1][c];
                // Left
                if c as i32 - 1 >= 0 && grid[r][c - 1] == '^' {
                    count += timelines[r - 1][c - 1];
                }
                // Right
                if c + 1 < grid[r].len() && grid[r][c + 1] == '^' {
                    count += timelines[r - 1][c + 1];
                }
                timelines[r][c] = count;
                row_count += count;
            }
        }

        println!("Row {} contains {} paths", r, row_count);
    }

    row_count
}

/// Tracks a trachyon beam down the grid
/// Returns the number of splits
fn track_classical_trachron(grid: &mut Vec<Vec<char>>) -> usize {
    // Index bounds
    let rows = grid.len();
    let cols = grid[0].len();

    // Tracking number of times a beam gets split
    let mut splits: usize = 0;

    // Notes:
    // There are no splitters on the grid column borders
    // There are no side-by-side splitters

    // Track the beams down the grid
    for r in 1..rows {
        // Iterate through each row (starting with the second) to see beam travels
        for c in 0..cols {
            // Two rules for beam travel:
            // 1. If empty space '.', any beam (or source 'S') above travels down to this space
            if grid[r][c] == '.' && (grid[r - 1][c] == '|' || grid[r - 1][c] == 'S') {
                grid[r][c] = '|';
            }

            // 2. If splitter '^', any beam above gets placed on right and left of the splitter
            // Update the split counter
            if grid[r][c] == '^' && grid[r - 1][c] == '|' {
                splits += 1;
                grid[r][c - 1] = '|';
                grid[r][c + 1] = '|';
            }
        }
    }

    splits
}
