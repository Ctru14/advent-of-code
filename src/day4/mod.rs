pub(crate) fn solve_day4() {
    // Get the map of paper rolls
    let binding = include_str!("day4-input.txt");
    let input = binding.trim();
    let lines = input.lines();

    // Create the 2D map of paper
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }

    let mut roll_removed_count: u32 = 0;
    let mut rolls_removed_pass = true;
    let mut passes: u32 = 0;

    while rolls_removed_pass {
        rolls_removed_pass = false;

        let mut idx_to_remove = Vec::<(usize, usize)>::new();
        // Count how many rolls of paper are accessible (fewer than 4 adjascent rolls)
        for (y, str) in map.iter().enumerate() {
            for (x, &char) in str.iter().enumerate() {
                // If this is a roll of paper, count how many rolls are adjascent to it
                if char == '@' {
                    let num_adj = count_adj_num(x, y, &map);
                    if num_adj < 4 {
                        rolls_removed_pass = true;
                        println!("Free roll found! x:{}, y:{}, count:{}", x, y, num_adj);
                        roll_removed_count += 1;

                        // Stage roll to be removed by marking it with *
                        idx_to_remove.push((y, x));
                    }
                }
            }
        }

        // Remove the staged indices
        for (y, x) in idx_to_remove {
            map[y][x] = '*';
        }

        passes += 1;
        println!(
            "After pass {}: {} rolls removed",
            passes, roll_removed_count
        );
    }

    println!("There are {} free paper rolls!", roll_removed_count);
}

fn count_adj_num(x: usize, y: usize, map: &Vec<Vec<char>>) -> u32 {
    if map[y][x] != '@' {
        panic!(
            "Adjascent count called on non-paper! x:{}, y:{}, char:{}",
            x, y, map[x][y]
        );
    }

    let mut count: u32 = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let x_idx: i32 = x as i32 + i;
            let y_idx: i32 = y as i32 + j;
            if x_idx >= 0 && y_idx >= 0 {
                if let Some(str) = map.get(y_idx as usize) {
                    if let Some(&adj) = str.get(x_idx as usize) {
                        if adj == '@' {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count - 1 // Subtract 1 for the itself
}
