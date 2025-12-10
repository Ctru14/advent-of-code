use std::vec;
use std::fmt::Display;

pub(crate) fn solve_day9() {
    // Get the input: List of coordinates of red tiles to make rectangles
    // let input = include_str!("day9-input.txt");
    let input = include_str!("day9-test.txt");

    let mut points: Vec<Point> = parse_input(input);

    let max_area = largest_area(&points);

    println!("The largest are rectangle you can make is {}", max_area); // 4773451098

    let (x_map, y_map) = condense_point_space(&mut points);
    let mut lines: Vec<Line> = form_lines(&points);

    let mut squares = populate_map(&mut lines);

    println!("\nx_map (len {}): {:?}", x_map.len(), x_map);
    println!("\ny_map (len {}): {:?}", y_map.len(), y_map);

    println!("\nPoints: {:?}", points);
    println!("\nLines: {:?}", lines);
    print_squares(&squares);
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Red,
    Green,
    Outside,
    Unknown
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Red => write!(f, "#"),
            Square::Green => write!(f, "X"),
            Square::Outside => write!(f, "o"),
            Square::Unknown => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    map_x: usize,
    map_y: usize,
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
            map_x: 0, // Will fill in later
            map_y: 0, // Will fill in later
        }
    }
}

#[derive(Debug)]
struct Line<'a> {
    current: (usize, usize),
    update: (usize, usize),
    end: &'a Point,
    p1: &'a Point,
    p2: &'a Point,
}

impl<'a> Iterator for Line<'a> {
    type Item = (usize, usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.current = (self.current.0 + self.update.0, self.current.1 + self.update.1);
        if self.current.0 == self.end.map_x && self.current.1 == self.end.map_y {
            None
        } else {
            Some(self.current)
        }
    }
}

impl<'a> Line<'a> {
    fn new(p1: &'a Point, p2: &'a Point) -> Self {
        // Since one coordinate is the same, we need to determine which one's map coordinate is higher
        let update_test = ((p1.map_x as i64 - p2.map_x as i64).signum(), (p1.map_y as i64 - p2.map_y as i64).signum());
        let (current, end): ((usize, usize), &Point) = if update_test.0 < 0 || update_test.1 < 0 {
            // P2 has a higher coordinate - use P1
            ((p1.map_x, p1.map_y), p2)
        } else {
            // P1 has a higher coordinate - use P2
            ((p2.map_x, p2.map_y), p1)
        };
        let update = (update_test.0.abs() as usize, update_test.1.abs() as usize);

        Self { current, update, end, p1, p2 }
    }
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

fn populate_map<'a>(lines: &mut Vec<Line<'a>>) -> Vec<Vec<Square>> {
    let count = lines.len();
    let mut squares: Vec<Vec<Square>> = vec![vec![Square::Unknown; count]; count];

    // Fill in the squares with red and green tiles from the points
    for line in lines {
        // Set the point's squares as red
        squares[line.p1.map_x][line.p1.map_y] = Square::Red;
        squares[line.p2.map_x][line.p2.map_y] = Square::Red;

        // Set all the squares on the line between as green
        for point in line {
            squares[point.1][point.0] = Square::Green;
        }
    }

    squares
}

/// Maps the points to a condensed space of coordinates corresponding to an index of the map
/// 
/// Ex:
///  012345678 ->  012
/// 0....#....    0.#.
/// 1......#..    1..#
/// 2.........    2#..
/// 3.#.......
/// 
fn condense_point_space(points: &mut Vec<Point>) -> (Vec<i64>, Vec<i64>) {
    let count = points.len();
    let mut x_map: Vec<i64> = Vec::with_capacity(count/2 + count/8);
    let mut y_map: Vec<i64> = Vec::with_capacity(count/2 + count/8);

    let mut points_sorted: Vec<Point> = points.clone();
    
    // Map X coordinates to condensed point space
    points_sorted.sort_by(|a,b| a.x.cmp(&b.x));
    for point_x in &points_sorted {
        if let Some(&last_x) = x_map.get(x_map.len().saturating_sub(1)) && point_x.x == last_x {
            // Use the existing last x_map point, so do nothing here
        } else {
            // Append a new x_map from this point
            x_map.push(point_x.x);
        }
    }

    // Map Y coordinates to condensed point space
    points_sorted.sort_by(|a,b| a.y.cmp(&b.y));
    for point_y in points_sorted {
        if let Some(&last_y) = y_map.get(y_map.len().saturating_sub(1)) && point_y.y == last_y {
            // Use the existing last x_map point, so do nothing here
        } else {
            // Append a new y_map from this point
            y_map.push(point_y.y);
        }
    }

    // Populate the map coordinates of all the points
    for point in points {
        point.map_x = x_map.iter().position(|&b| b == point.x).unwrap();
        point.map_y = y_map.iter().position(|&b| b == point.y).unwrap();
    }

    (x_map, y_map)
}

fn form_lines<'a>(points: &'a Vec<Point>) -> Vec<Line<'a>> {
    let count = points.len();
    let mut lines: Vec<Line<'_>> = Vec::with_capacity(count);

    for i in 0..count {
        lines.push(Line::new(&points[i % count], &points[(i + 1) % count]));
    }

    lines
}

fn parse_input(input: &str) -> Vec<Point> {
    let lines = input.lines();
    let mut points: Vec<Point> = Vec::with_capacity(500);

    for line in lines {
        points.push(Point::from(line));
    }

    points
}

fn print_squares(squares: &Vec<Vec<Square>>) {
    for line in squares {
        for square in line {
            print!("{}", square);
        }
        println!();
    }
}