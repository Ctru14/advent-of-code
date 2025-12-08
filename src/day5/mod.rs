use std::str::Lines;
use std::{ops::RangeInclusive};

pub(crate) fn solve_day5() {
    // Get input list of ranges and IDs
    let binding = include_str!("day5-input.txt");
    // let binding  include_str!("day5-test.txt");
    let input = binding.trim();
    let lines: Lines<'_> = input.lines();

    // Lines contain both ranges 23-56 and inputs to test 2345
    let (fresh_ranges, test_ids) = get_ranges_and_ids(lines);

    // Count number of fresh IDs in the list
    let mut fresh_count: usize = 0;
    for id in test_ids {
        // Test if the ID is present in any of the ranges
        for range in &fresh_ranges {
            if range.contains(&id) {
                println!("{} is fresh from range {:?}", id, range);
                fresh_count += 1;
                break;
            }
        }
    }

    println!("There are {} fresh items", fresh_count);

    println!("\n\nPart 2: Total available fresh IDs");
    // Now, count how many total fresh IDs there can be
    // let unique = get_unique_ids_brute_force(&fresh_ranges);
    let trimmed_fresh_ranges: Vec<RangeInclusive<usize>> = get_nonoverlapping_ranges(&fresh_ranges);

    // Count unique IDs from the ranges
    let mut unique_count: usize = 0;
    for range in trimmed_fresh_ranges {
        let range_count = range.end() - range.start() + 1;
        unique_count += range_count;
        println!(
            "Range {:?} has {} entries. New count = {}",
            range, range_count, unique_count
        );
    }

    println!("There are {} unique fresh IDs", unique_count);
}

fn get_ranges_and_ids(lines: Lines<'_>) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut fresh_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    let mut test_ids: Vec<usize> = Vec::new();
    for line in lines {
        // Only parse lines with ranges
        if let Some(idx) = line.find('-') {
            let lower = usize::from_str_radix(&line[..idx], 10).unwrap();
            let upper = usize::from_str_radix(&line[idx + 1..], 10).unwrap();
            fresh_ranges.push(lower..=upper);
        }

        // Only push lines with successful parsing
        if let Ok(num) = usize::from_str_radix(line, 10) {
            test_ids.push(num);
        }
    }
    (fresh_ranges, test_ids)
}

///   ****   *****
///     ****
/// ***********
///               **
///
fn get_nonoverlapping_ranges(
    full_ranges: &Vec<RangeInclusive<usize>>,
) -> Vec<RangeInclusive<usize>> {
    let mut remaining_ranges: Vec<RangeInclusive<usize>> = full_ranges.clone();
    let mut trimmed_ranges: Vec<RangeInclusive<usize>> = Vec::with_capacity(full_ranges.len());
    let mut new_split_ranges: Vec<RangeInclusive<usize>> = Vec::with_capacity(full_ranges.len());
    let mut iter: usize = 1;

    while remaining_ranges.len() > 0 {
        println!(
            "\niter {}: Remaining ranges (len {}): {:?}",
            iter,
            remaining_ranges.len(),
            remaining_ranges
        );

        'start: for range in remaining_ranges {
            let mut new_trimmed = range.clone();

            // Check each range in our new list and trim to non-overlapping versions
            for current in &mut trimmed_ranges {
                match (
                    new_trimmed.start() < current.start(),
                    new_trimmed.end() > current.end(),
                ) {
                    (false, false) => {
                        // New:     ***    or  ***
                        // Old:   *******      ***
                        // New range is contained entirely within existing entry
                        // Stop processing this range
                        println!(
                            "Range {:?} is within {:?}, stop processing",
                            new_trimmed, current
                        );
                        continue 'start;
                    }
                    (true, false) => {
                        // New range start is lower than current range start, but end is less than end
                        // Check if there's overlap
                        if new_trimmed.end() >= current.start() {
                            // New:  *******    or  ******  ->  *** cut!
                            // Old:     *******       ****         *******
                            // Ranges overlap: Trim the new range to cut out overlap
                            print!(
                                "Trimming end! Current = {:?}, New = {:?}",
                                current, new_trimmed
                            );
                            new_trimmed = *new_trimmed.start()..=*current.start() - 1;
                            println!(", New trimmed: {:?}", new_trimmed);
                        } else {
                            // New: ****
                            // Old:      ****
                            // No overlap: keep processing new trimmed range
                        }
                    }
                    (false, true) => {
                        // New range end is higher than current range end, but start is greater than start
                        // Check if there's overlap
                        if new_trimmed.start() <= current.end() {
                            // New:      *******    or  ******  ->  cut!   **
                            // Old:  *******            ****         ******
                            // Ranges overlap: Trim the new range to cut out overlap
                            print!(
                                "Trimming start! Current = {:?}, New = {:?}",
                                current, new_trimmed
                            );
                            new_trimmed = current.end() + 1..=*new_trimmed.end();
                            println!(", New trimmed: {:?}", new_trimmed);
                        } else {
                            // New:       ****
                            // Old: ****
                            // No overlap: keep processing new trimmed range
                        }
                    }
                    (true, true) => {
                        // Current range is contained entirely within new range
                        // New:   ********
                        // Old:     ***
                        // Split into two ranges. Keep processing the low side here, come back to process the higher side
                        print!(
                            "Splitting! Current = {:?}, New = {:?}",
                            current, new_trimmed
                        );
                        new_split_ranges.push(*current.end() + 1..=*new_trimmed.end());
                        new_trimmed = *new_trimmed.start()..=*current.start() - 1;
                        println!(", New trimmed: {:?}", new_trimmed);
                    }
                }
            }

            println!("Adding {:?} to trimmed ranges", new_trimmed);
            trimmed_ranges.push(new_trimmed);
        }

        remaining_ranges = new_split_ranges;
        new_split_ranges = Vec::with_capacity(trimmed_ranges.len());
        iter += 1;
    }

    println!(
        "\nFinal trimmed ranges (len {}): {:?}\n",
        trimmed_ranges.len(),
        trimmed_ranges
    );
    trimmed_ranges
}

/// This did not complete after 20 minutes ... worth a try lol
fn _get_unique_ids_brute_force(full_ranges: &Vec<RangeInclusive<usize>>) -> Vec<usize> {
    let mut unique: Vec<usize> = Vec::with_capacity(full_ranges.len());

    for range in full_ranges {
        for num in range.clone() {
            if !unique.contains(&num) {
                unique.push(num);
            }
        }
    }

    unique
}
