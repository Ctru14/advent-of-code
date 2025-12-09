use std::str::Lines;

pub(crate) fn solve_day6() {
    // Get the math homework
    let binding = include_str!("day6-input.txt");
    // let binding = include_str!("day6-test.txt");
    let lines: Lines<'_> = binding.lines();

    let homework = parse_input(lines);

    let sum_p1 = Homework::solve(&homework.nums_p1, &homework.ops);
    println!("The total part 1 sum is: {}", sum_p1);

    let sum_p2 = Homework::solve(&homework.nums_p2, &homework.ops);
    println!("The total part 2 sum is: {}", sum_p2);
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<String> for Operation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim() {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            _ => Err("Invalid".to_string()),
        }
    }
}

#[derive(Debug)]
struct Homework {
    nums_p1: Vec<Vec<Option<u64>>>, // Numbers parsed the usual way
    nums_p2: Vec<Vec<Option<u64>>>, // Numbers parsed in R-L column way
    ops: Vec<Operation>,
}

impl Homework {
    fn solve(nums: &Vec<Vec<Option<u64>>>, ops: &Vec<Operation>) -> u64 {
        // println!("Solve: {:?}\n{:?}", nums, ops);

        // Parse through the grid and do the operation on the columns
        let cols = ops.len();
        let rows = nums.len();
        let mut accum: u64 = 0;
        println!(
            "Solve: cols = {}, rows = {}, #ops = {}",
            cols,
            rows,
            ops.len()
        );
        for r in 0..nums.len() {
            println!("  nums[{}] len = {}", r, nums[r].len());
        }

        for c in 0..cols {
            let mut col_res: u64 = nums[0][c].unwrap();
            for r in 1..rows {
                // println!("r={}/{}, c={}/{}, cur={} new={}", r, nums[r].len(), c, cols, col_res, nums[r][c]);
                if let Some(num) = nums[r][c] {
                    match ops[c] {
                        Operation::Add => col_res += num,
                        Operation::Multiply => col_res *= num,
                    }
                }
            }

            accum += col_res;
        }

        accum
    }
}

fn parse_input(lines: Lines) -> Homework {
    let mut nums_p1: Vec<Vec<Option<u64>>> = Vec::new();
    let mut ops: Vec<Operation> = Vec::new();
    let chars_p2: Vec<Vec<char>> = lines.clone().map(|s| s.chars().collect()).collect();

    // P1 nums and operations
    for line in lines {
        let first = char::from(line.as_bytes()[0]);
        let split = line.split_ascii_whitespace();
        // See if this line is a number or your operations
        if first == '*' || first == '+' {
            // Operations
            ops = split
                .map(|s| Operation::try_from(s.to_string()).unwrap())
                .collect();
            break; // That's all folks
        } else {
            // Number
            nums_p1.push(
                split
                    .map(|s| Some(u64::from_str_radix(s, 10).unwrap()))
                    .collect(),
            );
        }
    }

    // P2 nums
    let op_row: &Vec<char> = &chars_p2[chars_p2.len() - 1];
    let count_rows = nums_p1.len();
    let mut op_idx: usize = 0;
    let mut end_idx: usize;
    // Allocate P2 vec
    let mut nums_p2: Vec<Vec<Option<u64>>> = Vec::with_capacity(count_rows);
    for r in 0..count_rows {
        nums_p2.push(Vec::with_capacity(nums_p1[r].len()));
    }

    // let mut row: usize = 0;
    while op_idx < op_row.len() {
        end_idx = get_end_idx(op_row, op_idx);
        // println!("Op idx: {}, End idx: {}", op_idx, end_idx);
        // We have the bounds for our numbers. Parse the columns
        // 123
        //  4   => 35, 24, 1
        //   5
        // There's always a full column space between the numbers and next operation
        let nums = parse_col_num(op_idx, end_idx, count_rows, &chars_p2);
        for r in 0..count_rows {
            nums_p2[r].push(nums.get(r).copied());
        }
        op_idx = end_idx;
        // row += 1;
    }

    Homework {
        nums_p1,
        nums_p2,
        ops,
    }
}

fn parse_col_num(
    op_idx: usize,
    end_idx: usize,
    count_rows: usize,
    chars_p2: &Vec<Vec<char>>,
) -> Vec<u64> {
    let mut nums: Vec<u64> = Vec::new();
    for c in (op_idx..end_idx - 1).rev() {
        let mut num_str = String::with_capacity(count_rows);
        for r in 0..count_rows {
            let char = chars_p2[r][c];
            if char >= '0' && char <= '9' {
                num_str.push(char);
            }
        }
        // println!("Col {}: {}", c - op_idx, num_str);
        nums.push(u64::from_str_radix(&num_str, 10).unwrap());
    }
    nums
}

fn get_end_idx(op_row: &Vec<char>, cur: usize) -> usize {
    for (idx, &char) in op_row[cur + 1..].iter().enumerate() {
        if char == '+' || char == '*' {
            return cur + 1 + idx;
        }
    }

    op_row.len() + 1
}
