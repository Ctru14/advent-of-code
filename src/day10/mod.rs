pub(crate) fn solve_day10() {
    // Get the input: List of factory machine information
    let input = include_str!("day10-input.txt");
    // let input = include_str!("day10-test.txt");

    let machines = parse_input(input);

    let mut sum: usize = 0;
    for machine in machines {
        let min_buttons = machine.solve_lights();
        sum += min_buttons;
    }

    println!("Sum of number of hits: {}", sum);
}

#[derive(Clone, Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    /// Counts the lowest number of presses of groups of its buttons
    /// requred to turn on the desired indicator lights
    fn solve_lights(&self) -> usize {
        let mut count: usize = 0;

        // Keep checking all possible combinations of number of buttons pressed
        let n = self.buttons.len();
        for r in 1..=n {
            if self.check_ncr_button_groups(n, r) {
                count = r;
                break;
            }
        }

        assert_ne!(count, 0);
        count
    }

    /// Checks whether or not a selection of pressing n button groups
    /// can turn on the indicator lights to the desired pattern
    fn check_ncr_button_groups(&self, n: usize, r: usize) -> bool {
        // Ex: Pick 4
        // 1: 1 2 3 4
        // 2: 1(2 3 4) 2(3 4) 3(4)
        // 3: 1(2(3 4))
        for button_sets in NChooseRIter::new(n, r) {
            let mut lights_test = vec![false; self.lights.len()];
            for button_set in button_sets {
                for &button in &self.buttons[button_set] {
                    lights_test[button] = !lights_test[button];
                }
            }
            if lights_test == self.lights {
                return true;
            }
        }

        false
    }
}

struct NChooseRIter {
    n: usize,
    r: usize,
    next: Option<Vec<usize>>,
}

impl NChooseRIter {
    fn new(n: usize, r: usize) -> Self {
        let mut next: Vec<usize> = Vec::with_capacity(n);
        for i in 0..r {
            next.push(i);
        }
        Self {
            n,
            r,
            next: Some(next),
        }
    }

    /// Returns the number of positions that are at the end of the list
    /// Ex: for the 6 choose 3 list [0 1 2 3 4 5]
    ///     if your positions are:   * * *        => 0 move the last one (idx 2)
    ///     if your positions are:   *       * *  => 2 move the first one (idx 0)
    ///                                * * *           then move the others back
    ///     if your positions are:   *   *     *  => 1 move the middle one (idx 1)
    ///     if your positions are:         * * *  => 3 you're done
    fn how_many_are_at_the_end(&self) -> usize {
        let mut count: usize = 0;
        if let Some(next) = &self.next {
            for &pos in next.iter().rev() {
                if pos == self.n - 1 - count {
                    count += 1;
                } else {
                    break;
                }
            }
        } else {
            count = self.r;
        }

        count
    }
}

impl Iterator for NChooseRIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // Take your vector of n indices
        // Start at 0..n
        // Increment the last one until it gets to n-1
        // Then increment the second to last one till it gets to n-2... and so on
        let rtn = self.next.clone();
        let at_end = self.how_many_are_at_the_end();
        if let Some(next) = &mut self.next {
            if at_end == self.r {
                // End condition
                self.next = None
            } else {
                // Move the last index that's not at the end to the end
                let mut start_pos = next[self.r - 1 - at_end] + 1;
                for i in self.r - 1 - at_end..self.r {
                    next[i] = start_pos;
                    start_pos += 1;
                }

                // for idx in 0..self.r {
                //     println!(
                //         "n: {}, r: {}. idx: {}, self.r-1-idx: {}, self.n-1-idx: {}",
                //         self.n,
                //         self.r,
                //         idx,
                //         self.r - 1 - idx,
                //         self.n - 1 - idx
                //     );
                //     if self.next[self.r - 1 - idx] < self.n - 1 - idx {
                //         self.next[self.r - 1 - idx] += 1;
                //         return Some(self.next.clone());
                //     }
                // }
            }
        }

        rtn
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(' ').collect();

        // Lights [.##.]
        let mut lights: Vec<bool> = Vec::with_capacity(split[0].len() - 2);
        // Cut out [ ]
        for &char in &split[0].chars().collect::<Vec<char>>()[1..split[0].len() - 1] {
            if char == '.' {
                lights.push(false);
            } else if char == '#' {
                lights.push(true);
            } else {
                panic!("Unexpected lights character: {}", char);
            }
        }

        // Buttons: (3) (1,3) (5,2,3) ..
        let mut buttons: Vec<Vec<usize>> = Vec::with_capacity(split.len() - 2);
        for &button_str in &split[1..split.len() - 1] {
            let button_set: Vec<usize> = button_str[1..button_str.len() - 1]
                .split(',')
                .map(|s| usize::from_str_radix(s, 10).unwrap())
                .collect();
            buttons.push(button_set);
        }

        // Joltage: {3,5,4,7}
        let mut joltage: Vec<usize> = split[split.len() - 1][1..split[split.len() - 1].len() - 1]
            .split(',')
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect();

        Self {
            lights,
            buttons,
            joltage,
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::with_capacity(188);

    for line in input.lines() {
        machines.push(Machine::from(line.trim()));
    }

    machines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ncr_iter() {
        assert_eq!(
            NChooseRIter {
                n: 6,
                r: 3,
                next: Some(vec![1, 2, 5])
            }
            .how_many_are_at_the_end(),
            1
        );

        assert_eq!(
            NChooseRIter {
                n: 6,
                r: 3,
                next: Some(vec![2, 4, 5])
            }
            .how_many_are_at_the_end(),
            2
        );

        assert_eq!(
            NChooseRIter {
                n: 6,
                r: 3,
                next: Some(vec![3, 4, 5])
            }
            .how_many_are_at_the_end(),
            3
        );

        assert_eq!(
            NChooseRIter {
                n: 6,
                r: 3,
                next: None
            }
            .how_many_are_at_the_end(),
            3
        );

        let ncr42 = NChooseRIter::new(4, 2);
        assert_eq!(ncr42.how_many_are_at_the_end(), 0);

        let ncr42_expected = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
        ];

        for (idx, item) in NChooseRIter::new(4, 2).enumerate() {
            assert_eq!(ncr42_expected[idx], item);
        }
    }
}
