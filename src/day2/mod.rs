use std::ops::RangeInclusive;

pub(crate) fn solve_day2() {
    // Get file input
    let binding = include_str!("day2-input.txt");
    let input = binding.trim(); // Remove trailing whitespace

    // Split input string on commas to get ranges
    // i.e. 6161588270-6161664791,128091420-128157776,306-494,...
    let ranges = input.split(',');

    // Tracks the total sum of all invalid IDs
    let mut accum_twice: u64 = 0;
    let mut accum_any: u64 = 0;

    for range_str in ranges {
        let range = match parse_range_str(range_str) {
            Some(r) => r,
            None => continue,
        };

        for num in range {
            if is_twice_repeated_sequence(num) {
                accum_twice += num;
                // println!("{} is a twice repeated digit within range {}: sum = {}", num, range_str, accum_twice);
            }

            if is_n_repeated_sequence(num) {
                accum_any += num;
                println!(
                    "{} is an N-repeated digit within range {}: sum = {}",
                    num, range_str, accum_twice
                );
            }
        }
    }

    println!(
        "The final total sum of the twice-repeated invalid IDs is: {}",
        accum_twice
    );
    println!(
        "The final total sum of the any-repeated invalid IDs is: {}",
        accum_any
    );
}

/// Checks if the number is a twice repeated string of digits
/// ex. 11, 22, 1010, 123123
fn is_twice_repeated_sequence(num: u64) -> bool {
    let num_str = num.to_string();

    if num_str.len() % 2 == 0 {
        // Even number of digits - check if both halves are equal
        num_str[..num_str.len() / 2] == num_str[num_str.len() / 2..]
    } else {
        // Odd number of digits cannot be repeated
        false
    }
}

/// Checks if a number is a repeated string of digits of any length
/// Ex: 11, 123123123, ...
fn is_n_repeated_sequence(num: u64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();

    // Ex: 1212121212 - 5 repeats of '12'
    // Check 0..2, 2..4, 4..6

    // println!("num: {}, len: {}", num_str, len);

    // Check for a repeating string of i digits
    'check_i: for i in 1..len {
        // Only check numbers if there's an even division of them
        if len % i == 0 {
            let test_pattern = &num_str[..i];
            // println!("i = {}, test pattern: {}", i, test_pattern);
            for start in 0..(len / i) {
                if &num_str[start * i..start * i + i] != test_pattern {
                    // println!("indices {}..{} = {}, does not match", start*i, start*i+i, &num_str[start*i..start*i+i]);
                    continue 'check_i;
                }
            }

            // All patterns matched if we got here
            return true;
        }
    }

    false
}

fn parse_range_str(range: &str) -> Option<RangeInclusive<u64>> {
    let nums: Vec<&str> = range.split('-').collect();
    if let [lo_str, hi_str] = nums[0..2] {
        println!("{} - {}", lo_str, hi_str);
        let lo = u64::from_str_radix(lo_str, 10).unwrap();
        let hi = u64::from_str_radix(hi_str, 10).unwrap();
        Some(lo..=hi)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::day2::is_n_repeated_sequence;

    #[test]
    fn test_any_repeated() {
        assert!(is_n_repeated_sequence(123123123));
    }
}
