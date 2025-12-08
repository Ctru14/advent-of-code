pub(crate) fn solve_day1() {
    println!("Solving the Advent of Code, day 1!");

    // Get file input
    let input = include_str!("day1-password.txt");

    // Split by lines so each live gives an instruction (i.e. R23, L1)
    let split = input.split("\n");

    // Safe starts at 50
    let mut pos: i32 = 50;
    let mut _lines = 0;
    let mut zero_point_count: u32 = 0;
    let mut zero_cross_count: u32 = 0;

    for line in split {
        // Need at least [L/R][#] for an instruction
        if line.len() < 2 {
            continue;
        }

        // Move safe value by parsed amount
        let turn: i32 = parse_line_for_instruction(line);
        let zero_cross: u32 = count_zero_clicks(pos, turn);

        // Update position
        pos += turn;
        pos %= 100;
        if pos < 0 {
            pos += 100;
        }
        // println!("Line {}: {}. Moved to {}", lines, line, pos);

        // Increment however many times it lands on 0
        if pos == 0 {
            zero_point_count += 1;
            // println!("Safe landed on 0! #{}", zero_point_count);
        }

        // Increment number of times it crossed zero
        zero_cross_count += zero_cross;

        if zero_cross > 0 {
            // println!("Safe crossed zero {} time(s)", zero_cross);
        }

        _lines += 1;
    }

    println!(
        "The safe landed on 0 a total of {} times!",
        zero_point_count
    );
    println!("The safe crossed 0 a total of {} times!", zero_cross_count);
}

fn parse_line_for_instruction(line: &str) -> i32 {
    let (dir_str, mag_str) = line.split_at(1);
    let mag = i32::from_str_radix(mag_str, 10).unwrap();
    let dir = match dir_str {
        "R" => 1,
        "L" => -1,
        _ => panic!("Invalid direction: {}", dir_str),
    };
    mag * dir
}

fn count_zero_clicks(pos: i32, turn: i32) -> u32 {
    let mut clicks: i32 = (pos + turn) / 100;
    if clicks < 0 {
        clicks *= -1;
    }
    if pos + turn == 0 {
        clicks += 1;
    }
    if pos.signum() == -1 * turn.signum() && pos.abs() < turn.abs() {
        clicks += 1;
    }
    clicks as u32
}

#[cfg(test)]
mod test {
    use super::count_zero_clicks;

    #[test]
    fn test_zero_clicks() {
        assert_eq!(count_zero_clicks(1, 1), 0);
        assert_eq!(count_zero_clicks(1, 100), 1);
        assert_eq!(count_zero_clicks(1, 1000), 10);
        assert_eq!(count_zero_clicks(95, 60), 1);
        assert_eq!(count_zero_clicks(0, 99), 0);
        assert_eq!(count_zero_clicks(50, 1000), 10);
        assert_eq!(count_zero_clicks(50, -60), 1);
        assert_eq!(count_zero_clicks(50, -68), 1);
        assert_eq!(count_zero_clicks(55, -55), 1);
    }
}
