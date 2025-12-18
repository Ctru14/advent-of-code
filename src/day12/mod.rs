use std::array;

pub(crate) fn solve_day12() {
    // Get the input: List of factory machine information
    let input = include_str!("day12-input.txt");
    // let input = include_str!("day12-test.txt");

    let tree = parse_input(input);
    println!("{:?}", tree);

    // 414: HAH THAT WORKED!!
    let count = tree.count_fitting_arrangements();
    println!("{}/{} spaces can fit", count, tree.num_total_arrangements())
}

#[derive(Debug, Default, Clone, Copy)]
struct Present([[char; 3]; 3]);

impl Present {
    /// Counts how many filled blocks are in the present
    fn area(&self) -> usize {
        let mut count = 0;
        for line in self.0 {
            for char in line {
                if char == '#' {
                    count += 1;
                }
            }
        }
        count
    }
}

impl<'a> From<&'a str> for Present {
    fn from(value: &'a str) -> Self {
        let mut present = Self::default();
        let mut lines = value.lines();
        let _ = lines.next(); // Skip index line
        for presence in &mut present.0 {
            let next_line = lines.next().unwrap().chars();
            presence
                .iter_mut()
                .zip(next_line)
                .for_each(|(to, from)| *to = from);
        }

        present
    }
}

#[derive(Debug)]
struct TreeSpace {
    grid: (usize, usize),
    /// Each index corresponds to how many of each present shape need to fit in the grid
    num_each_shape: [usize; 6],
}

impl TreeSpace {
    fn area(&self) -> usize {
        self.grid.0 * self.grid.1
    }

    /// Returns whether or not all the presents can fit in that grid
    fn evaluate_fit(&self, presents: &[Present; 6]) -> bool {
        // Test 1: Check if the area of each present is within the bounds of the grid
        let mut area_count: usize = 0;
        for (present, present_count) in presents.iter().zip(self.num_each_shape) {
            area_count += present.area() * present_count;
        }

        // Return whether the total area fits in bounds
        let area_test = area_count <= self.area();
        println!("Grid area: {}, Presents area: {}, Can it fit? {}", self.area(), area_count, area_test);
        area_test
    }
}

impl From<&str> for TreeSpace {
    fn from(value: &str) -> Self {
        // 12x5: 1 0 1 0 2 2
        let mut split = value.split_ascii_whitespace();
        let mut grid_split = split.next().unwrap().split(['x', ':']);

        let grid = (
            usize::from_str_radix(grid_split.next().unwrap(), 10).unwrap(),
            usize::from_str_radix(grid_split.next().unwrap(), 10).unwrap(),
        );

        let num_shapes: [usize; 6] =
            array::from_fn(|_| usize::from_str_radix(split.next().unwrap(), 10).unwrap());

        Self { grid, num_each_shape: num_shapes }
    }
}

#[derive(Debug)]
struct Tree {
    presents: [Present; 6],
    spaces: Vec<TreeSpace>,
}

impl Tree {
    /// Return the count of how many arrangements can fit in their grid
    fn count_fitting_arrangements(&self) -> usize {
        let mut count = 0;
        for space in &self.spaces {
            if space.evaluate_fit(&self.presents) {
                count += 1;
            }
        }
        count
    }

    /// Returns the number of proposed presents arrangements
    fn num_total_arrangements(&self) -> usize {
        self.spaces.len()
    }
}

fn parse_input(input: &str) -> Tree {
    let mut presents: [Present; 6] = [Present::default(); 6];
    let mut spaces: Vec<TreeSpace> = Vec::new();

    // There are always 6 presents at the start
    let mut newline_split = input.split("\n\n");
    // let mut newline_split = input.split("\r\n\r\n"); // Test
    for i in 0..6 {
        let next = newline_split.next();
        println!("next: {:?}", next);
        presents[i] = Present::from(next.unwrap());
    }

    // After the presents, all the lines are the grid problems
    let treespaces_lines = newline_split.next().unwrap().lines();
    for line in treespaces_lines {
        spaces.push(TreeSpace::from(line));
    }

    Tree { presents, spaces }
}
