use z3::{Solver, ast::Int};

pub(crate) fn solve_day10() {
    // Get the input: List of factory machine information
    let input = include_str!("day10-input.txt");
    // let input = include_str!("day10-test.txt");

    let machines = parse_input(input);

    // Part 1: Toggle indicator lights
    let mut indicator_lights_sum: usize = 0;
    for machine in &machines {
        let min_buttons = machine.solve_lights();
        indicator_lights_sum += min_buttons;
    }

    println!(
        "Indicator lights: Sum of number of hits: {}",
        indicator_lights_sum
    );

    // Part 2: Increase Joltage
    let mut joltage_sum: usize = 0;
    for machine in &machines {
        let min_buttons = machine.solve_joltage();
        joltage_sum += min_buttons;
    }

    println!("Joltage: Sum of number of hits: {}", joltage_sum);
}

#[derive(Clone, Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    /// Return the lowest number of presses of groups of buttons required to
    /// increase the joltage to the desired amounts
    fn solve_joltage(&self) -> usize {
        // Turn the buttons and desired joltage into a matrix
        let mut mat = self.get_joltage_matrix();

        // Reduce the matrix to speed up solver
        mat.gauss_jordan_elimination();

        // Set up Z3 solver
        let solver = Solver::new();
        let vars: Vec<Int> = Vec::with_capacity(mat.cols()-1);
        for i in 0..mat.cols()-1 {
            vars.push(Int::fresh_const(format!("v{}", i)));
        }

        // Constraints
        // Vars cannot be negative
        vars.iter().for_each(|var| solver.assert(var.ge(0)));
        
        // Add any rows to the solver that have more than 2 item remaining
        let mut count = 0;
        let mut some_solve_count = false;
        for row in mat.0 {
            let mut nonzero_count: usize = 0;
            row.iter().for_each(|&val| {
                if val != 0 {
                    nonzero_count += 1;
                }
            });
            match nonzero_count {
                0 => {}, // Ignore empty row
                1 | 2 => {
                    // Solved variable - add constant
                    let (idx, _val) = Matrix::get_first_nonzero_value(&row).unwrap();
                    solver.assert(vars[idx].eq(row[row.len()-1]));
                }
                _ => {
                    // Add this equation to the solver
                    solver.assert((vars.iter().enumerate().for_each(|(idx, var)| var * row[idx])).eq(row[row.len()-1]));
                }
            }
        }

        // Solve and check solutions
        let mut min_presses = usize::MAX;
        for solution in solver.solutions(vars, true).take(100) {
            let solution: Vec<usize> = solution.iter().map(Int::as_u64).map(Option::unwrap).collect();
            let sum: usize = solution.iter().sum();
            if sum < min_presses {
                min_presses = sum;
            }
        }

        println!("Entry can be solved in {} presses", min_presses);
        min_presses
    }

    /// Creates a matrix of the button presses and the desired joltage
    /// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    ///         a    b    c    d     e     f
    /// a b c d e f  =
    /// 0 0 0 0 1 1  3
    /// 0 1 0 0 0 1  5
    /// 0 0 1 1 1 0  4
    /// 1 1 0 1 0 0  7
    fn get_joltage_matrix(&self) -> Matrix {
        let rows = self.joltage.len();
        let num_buttons = self.buttons.len();
        let cols = num_buttons + 1; // Each button is a column + the output joltage
        let mut mat: Vec<Vec<i32>> = Vec::with_capacity(rows);

        for r in 0..rows {
            let mut row: Vec<i32> = Vec::with_capacity(cols);
            for button in 0..num_buttons {
                if self.buttons[button].contains(&r) {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            row.push(self.joltage[r] as i32);
            mat.push(row);
        }

        Matrix(mat)
    }

    /// Return the lowest number of presses of groups of buttons required to
    /// increase the joltage to the desired amounts
    /// Checks all possible values starting from max of the joltage values
    /// Will never complete before the heat death of the universe
    fn _solve_joltage_brute_force(&self) -> usize {
        let mut count = 0;

        // Keep checking all possible combinations of number of buttons pressed
        // Start with the max of the joltage numbers, the answer will never be less
        let mut presses = *self.joltage.iter().max().unwrap();
        loop {
            if self._check_joltage_counters_brute_force(presses) {
                count = presses;
                break;
            }
            presses += 1;
        }

        count
    }

    /// Check if the joltage levels can be achieved by the number of presses
    fn _check_joltage_counters_brute_force(&self, presses: usize) -> bool {
        for button_sets in _SelectNCountIter::_new(self.buttons.len(), presses) {
            // Index of each button_sets corresponds to index of each button to press,
            // value corresponds to how many times to press it
            let mut joltage_test = vec![0; self.joltage.len()];
            // Each iteration is +1 button press
            for button_set_to_press in button_sets {
                for &button in &self.buttons[button_set_to_press] {
                    joltage_test[button] += 1;
                }
            }

            if joltage_test == self.joltage {
                println!("Min number found: {}", presses);
                return true;
            }
        }

        false
    }

    /// Counts the lowest number of presses of groups of its buttons required to
    /// turn on the desired indicator lights
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct Matrix(Vec<Vec<i32>>);

impl Matrix {
    /// Returns the number of rows
    fn rows(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of columns
    fn cols(&self) -> usize {
        self.0[0].len()
    }

    /// Performs Gauss-Jordan elimination to reduce a matrix
    fn gauss_jordan_elimination(&mut self) {
        let rows = self.0.len();

        // Current index of the column which is being transformed into a 1
        let mut pivot_row: usize;
        let mut pivot_col: usize = 0;

        let mut rescale_needed: bool = false;

        for r in 0..rows {
            // 1. Find the next available pivot column
            // Check each column for the presence of a nonzero value
            (pivot_row, pivot_col) = match self.next_available_pivot_column(r, pivot_col) {
                Some((row, col)) => (row, col),
                None => {
                    // Unable to find another pivot value. Ending
                    return;
                }
            };

            // 2. Swap a row with said column index present into the current row
            self.0.swap(r, pivot_row);
            // Now the row with the pivot column is at index r

            // 3. Scale the row if needed so the pivot column entry is equal to 1
            let rescale_result = Self::try_rescale_row(&mut self.0[r]);

            // 4. Add/Subtract this row from any following rows which also contain our pivot column
            // Only attempt this subtraction if the previous rescale attempt passed
            // so the pivot column value will be 1
            if rescale_result.is_ok() {
                self.subtract_out_pivot(r, pivot_col);
            }

            // Attempt to rescale any previously failed row
            if rescale_needed {
                rescale_needed = self.try_rescale_and_subtract_first_n_rows(r).is_err();
            }

            // Try a full rescale on next attempt if this row failed
            if rescale_result.is_err() {
                rescale_needed = true;
            }
        }
    }

    /// Returns the row and column index of the next available pivot column
    /// beginning from the provided indices
    fn next_available_pivot_column(
        &self,
        start_row: usize,
        start_col: usize,
    ) -> Option<(usize, usize)> {
        let rows = self.0.len();
        let cols = self.0[0].len();

        // Last column is the result, do not count it
        for check_col in start_col..cols - 1 {
            for check_row in start_row..rows {
                if self.0[check_row][check_col] != 0 {
                    return Some((check_row, check_col));
                }
            }
        }

        None
    }

    /// Goes through the first so many rows and attempts to scale them
    /// so that the first nonzero value in that row is 1
    ///
    /// Return: Result whether all rows are properly rescaled
    fn try_rescale_and_subtract_first_n_rows(&mut self, n: usize) -> Result<(), ()> {
        let mut res = Ok(());
        for r in 0..n {
            // Check if this row needs a rescale
            if let Some((pivot_col, val)) = Self::get_first_nonzero_value(&self.0[r])
                && val != 1
            {
                if Self::try_rescale_row(&mut self.0[r]).is_ok() {
                    // Row rescaled! Now subtract with it
                    self.subtract_out_pivot(r, pivot_col);
                } else {
                    res = Err(());
                }
            };
        }

        res
    }

    /// Tries to rescale the row so that the first nonzero value is 1
    /// Row of all zeros is properly scaled and returns Ok(())
    ///
    /// Return: Result whether the row was properly rescaled
    fn try_rescale_row(row: &mut Vec<i32>) -> Result<(), ()> {
        if let Some((_idx, scale_val)) = Self::get_first_nonzero_value(row) {
            match scale_val {
                1 => Ok(()), // As expected
                -1 => {
                    // Multiply all values of the row by -1
                    row.iter_mut().for_each(|val| *val *= -1);
                    Ok(())
                }
                0 => {
                    panic!("Rows with all 0s should fail the option check");
                }
                _ => {
                    // Uhhhh we're only using integers so this could get bad
                    if row.iter().any(|val| val % scale_val != 0) {
                        Err(())
                    } else {
                        row.iter_mut().for_each(|val| *val /= scale_val);
                        Ok(())
                    }
                }
            }
        } else {
            // All 0s - this is properly scaled
            Ok(())
        }
    }

    /// Subtracts out a pivot column using the equation in the specified row
    fn subtract_out_pivot(&mut self, pivot_row: usize, pivot_col: usize) {
        let rows = self.0.len();
        let cols = self.0[0].len();

        // This row must be properly scaled
        assert_eq!(self.0[pivot_row][pivot_col], 1);

        for target_row in 0..rows {
            let pivot_val = self.0[target_row][pivot_col];
            // println!("Pivot val: {}", pivot_val);
            if target_row != pivot_row && pivot_val != 0 {
                for c in 0..cols {
                    self.0[target_row][c] -= pivot_val * self.0[pivot_row][c];
                }
            }
        }
    }

    /// Returns the first nonzero value in the list along with its index
    fn get_first_nonzero_value(row: &Vec<i32>) -> Option<(usize, i32)> {
        for (idx, &val) in row.iter().enumerate() {
            if val != 0 {
                return Some((idx, val));
            }
        }

        None
    }
}

struct _SelectNCountIter {
    n: usize,
    /// Vec length of count, where each of them correspond to the index of each selection
    /// Each value will range from 0..=n
    next: Option<Vec<usize>>,
}

impl _SelectNCountIter {
    fn _new(n: usize, count: usize) -> Self {
        let next: Vec<usize> = vec![0; count];
        Self {
            n,
            next: Some(next),
        }
    }
}

impl Iterator for _SelectNCountIter {
    type Item = Vec<usize>;

    /// Ex: 5 choose 3
    /// 0 0 0
    /// 0 0 1
    /// ...
    /// 0 0 5
    /// 0 1 0 Overflow!
    /// 0 1 1
    ///  ...
    /// 0 1 5
    /// 0 2 0 Overflow!
    fn next(&mut self) -> Option<Self::Item> {
        // Increment the lowest index
        // If its new value is greater than the count, increment the next one and reset these
        // End when all values are n
        let n = self.n;
        let rtn = self.next.clone();
        if let Some(next) = &mut self.next {
            for val in next {
                *val += 1;
                if *val >= n {
                    *val = 0;
                } else {
                    break;
                }
            }
        }

        // Check end condition
        if let Some(next) = &self.next {
            if next.iter().sum::<usize>() == 0 {
                self.next = None;
            }
        }

        rtn
    }
}

struct NChooseRIter {
    n: usize,
    r: usize,
    /// Length r, values ranging from 0..n, corresponding to which indices to select
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
        let joltage: Vec<usize> = split[split.len() - 1][1..split[split.len() - 1].len() - 1]
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

fn print_matrix(Matrix(mat): &Matrix) {
    println!("Matrix {} x {}:", mat.len(), mat[0].len());
    for row in mat {
        println!("{:?}", row);
    }
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

    #[test]
    fn test_selectncount_iter() {
        let select1of3iter = _SelectNCountIter::_new(3, 1);
        let s13_expected = vec![vec![0], vec![1], vec![2]];

        for (idx, item) in select1of3iter.enumerate() {
            assert_eq!(s13_expected[idx], item);
        }
    }

    #[test]
    fn test_gauss_jordan_elimination() {
        let mut m1 = Matrix(vec![
            vec![1, 1, 1, 0, 0, 0, 0, 28],
            vec![0, 0, 1, 0, 0, 1, 0, 36],
            vec![1, 1, 0, 0, 1, 0, 1, 163],
            vec![1, 0, 0, 1, 1, 1, 0, 27],
            vec![0, 0, 1, 0, 0, 1, 1, 180],
            vec![0, 0, 0, 0, 1, 0, 0, 11],
            vec![1, 1, 1, 0, 0, 0, 1, 172],
            vec![1, 1, 0, 0, 1, 0, 0, 19],
        ]);
        m1.gauss_jordan_elimination();

        assert_eq!(
            m1,
            Matrix(vec![
                vec![1, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 1, 0, -1, 0, 0, 0, 8],
                vec![0, 0, 1, 0, 0, 0, 0, 20],
                vec![0, 0, 0, 0, 1, 0, 0, 11],
                vec![0, 0, 0, 0, 0, 1, 0, 16],
                vec![0, 0, 0, 0, 0, 0, 1, 144],
                vec![0, 0, 0, 0, 0, 0, 0, 0,],
                vec![0, 0, 0, 0, 0, 0, 0, 0,],
            ])
        );
        println!("Matrix 1 passed!\n");

        let mut m2 = Matrix(vec![
            vec![0, 1, 1, 0, 0, 1, 13],
            vec![0, 0, 1, 1, 1, 1, 43],
            vec![1, 1, 0, 0, 1, 0, 33],
            vec![1, 0, 0, 0, 1, 0, 29],
            vec![1, 1, 1, 0, 1, 0, 40],
            vec![1, 1, 0, 1, 0, 1, 41],
        ]);
        print_matrix(&m2);
        m2.gauss_jordan_elimination();
        print_matrix(&m2);

        let m2_expected = Matrix(vec![
            vec![1, 0, 0, 0, 0, 0, 15],
            vec![0, 1, 0, 0, 0, 0, 4],
            vec![0, 0, 1, 0, 0, 0, 7],
            vec![0, 0, 0, 1, 0, 0, 20],
            vec![0, 0, 0, 0, 1, 0, 14],
            vec![0, 0, 0, 0, 0, 1, 2],
        ]);

        assert_eq!(m2, m2_expected);
        println!("Matrix 2 passed!");

        let mut m3 = Matrix(vec![
            vec![0,0,0,0,1,1,3],
            vec![0,1,0,0,0,1,5],
            vec![0,0,1,1,1,0,4],
            vec![1,1,0,1,0,0,7],
        ]);
        m3.gauss_jordan_elimination();;

        let m3_expected = Matrix(vec![
            vec![1,0,0,1,0,-1,2],
            vec![0,1,0,0,0,1,5],
            vec![0,0,1,1,0,-1,1],
            vec![0,0,0,0,1,1,3]
        ]);

        assert_eq!(m3, m3_expected);
    }
}
