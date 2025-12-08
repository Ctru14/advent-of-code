pub(crate) fn solve_day3() {
    // Find the maximum joltage from all the batteries
    // Each battery has a 1-9 rating, and you turn on exactly two
    // Make the max possible joltage in each row
    // Ex: 987654321 = 98, 12345 = 45
    // And sum the total joltage for the answer

    // let binding = include_str!("day3-test.txt");
    let binding = include_str!("day3-input.txt");
    let input = binding.trim();

    let mut joltage: u64 = 0;

    for line in input.split("\n") {
        println!("\n{}", line);
        // let line_joltage = get_max_joltage_2(line);
        let line_joltage = get_max_joltage_n(line.trim(), 12);
        joltage += line_joltage;
        println!(
            "Line: {} J, new total {} J, {}",
            line_joltage, joltage, line
        );
    }

    println!("Test expected: 3121910778619 J");
    println!("Total Joltage: {} J", joltage);
}

fn _get_max_joltage_2(line: &str) -> u64 {
    // Get the farthest left max value, excluding the last digit
    let (tens, idx) = get_first_max(&line[..line.len() - 1], 0);

    // Get the max value from the string right of the idx
    let (ones, _) = get_first_max(&line[idx..], idx);

    u64::from_str_radix(format!("{}{}", tens, ones).as_str(), 10).unwrap()
}

fn get_max_joltage_n(line: &str, n: usize) -> u64 {
    let mut joltage = String::with_capacity(n);
    let mut idx: usize = 0;
    let mut char: char;

    for i in 0..n {
        (char, idx) = get_first_max(&line[idx..line.len() - n + i + 1], idx);
        joltage.push(char);
    }

    u64::from_str_radix(&joltage, 10).unwrap()
}

fn get_first_max(line: &str, start_idx: usize) -> (char, usize) {
    let max = line.chars().max().unwrap();

    for (idx, char) in line.char_indices() {
        if max == char {
            println!(
                "get_first_max({}): max={}, idx={}",
                line,
                max,
                idx + start_idx
            );
            return (max, start_idx + idx + 1);
        }
    }

    panic!("First index wasn't found!");
}
