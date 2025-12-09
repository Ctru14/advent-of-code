pub(crate) fn solve_day9() {
    // Get the input: List of coordinates of red tiles to make rectangles
    let input = include_str!("day9-input.txt");
    // let input = include_str!("day9-test.txt");

    let points: Vec<Point> = parse_input(input);

    let max_area = largest_area(&points);

    println!("The largest are rectangle you can make is {}", max_area); // 4773451098
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Point) -> u64 {
        (self.x - other.x + 1).abs() as u64 * (self.y - other.y + 1).abs() as u64
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.trim().split(',').collect();
        Self {
            x: i64::from_str_radix(split[0], 10).unwrap(),
            y: i64::from_str_radix(split[1], 10).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Line<'a> {
    p1: &'a Point,
    p2: &'a Point,
}

/// Brute force gets the largest area from any combination of points
fn largest_area(points: &Vec<Point>) -> u64 {
    let mut max_area: u64 = 0;
    let count = points.len();

    for l in 0..count {
        for r in 0..count {
            let area = points[l].area(&points[r]);
            if area > max_area {
                println!("New largest area found: {}:{:?} * {}{:?} = {}", l, points[l], r, points[r], area);
                max_area = area;
            }
        }
    }

    max_area
}

fn parse_input(input: &str) -> Vec<Point> {
    let lines = input.lines();
    let mut points: Vec<Point> = Vec::with_capacity(500);

    for line in lines {
        points.push(Point::from(line));
    }

    points
}
