use std::fmt::Display;
use std::vec;

pub(crate) fn solve_day9() {
    // Get the input: List of coordinates of red tiles to make rectangles
    let input = include_str!("day9-input.txt");
    // let input = include_str!("day9-test.txt");

    let mut points: Vec<Point> = parse_input(input);

    // let max_area = largest_area(&points);
    let areas = get_areas(&points);
    let max_area = areas[0].area;

    println!("The largest are rectangle you can make is {}", max_area); // 4773451098

    let (x_map, y_map) = condense_point_space(&mut points);
    let mut lines: Vec<Line> = form_lines(&points);

    let mut squares = populate_map(&mut lines, x_map.len(), y_map.len());

    println!("\nx_map (len {}): {:?}", x_map.len(), x_map);
    println!("\ny_map (len {}): {:?}", y_map.len(), y_map);

    // println!("\nPoints: {:?}", points);
    // println!("\nLines: {:?}", lines);

    let max_constrained_area = largest_constrained_area(&areas, &points, &mut squares);
    println!("Largest constrained area: {}", max_constrained_area);

    print_squares(&squares);
    // 1429043625 is too low
    // 1621520882 is too high
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Red,
    Green,
    Inside,
    Outside,
    Unknown,
    Rectangle,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Red => write!(f, "R"),
            Square::Green => write!(f, "G"),
            Square::Inside => write!(f, "i"),
            Square::Outside => write!(f, "o"),
            Square::Unknown => write!(f, " "),
            Square::Rectangle => write!(f, "*"),
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
        self.current = (
            self.current.0 + self.update.0,
            self.current.1 + self.update.1,
        );
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
        let update_test = (
            (p1.map_x as i64 - p2.map_x as i64).signum(),
            (p1.map_y as i64 - p2.map_y as i64).signum(),
        );
        let (current, end): ((usize, usize), &Point) = if update_test.0 < 0 || update_test.1 < 0 {
            // P2 has a higher coordinate - use P1
            ((p1.map_x, p1.map_y), p2)
        } else {
            // P1 has a higher coordinate - use P2
            ((p2.map_x, p2.map_y), p1)
        };
        let update = (update_test.0.abs() as usize, update_test.1.abs() as usize);

        Self {
            current,
            update,
            end,
            p1,
            p2,
        }
    }
}

fn largest_constrained_area(
    areas: &Vec<Area>,
    points: &Vec<Point>,
    squares: &mut Vec<Vec<Square>>,
) -> u64 {
    let mut max_area = 0;
    let count = points.len();
    let mut p1: usize = 0;
    let mut p2: usize = 0;

    for (idx, area) in areas.iter().enumerate() {
        println!("Checking area[{}]: {}", idx, area.area);
        if is_rectangle_within_polygon(&points[area.p1_idx], &points[area.p2_idx], &squares) {
            p1 = area.p1_idx;
            p2 = area.p2_idx;
            max_area = area.area;
            println!(
                "Largest constrained area found: {}:{:?} * {}:{:?} = {}",
                p1, points[area.p1_idx], p2, points[area.p2_idx], area.area
            );
            break;
        }
    }

    // for l in 0..count {
    //     for r in 0..count {
    //         let area = points[l].area(&points[r]);
    //         if area > max_area {
    //             // Check if all the points in this rectangle are within the polygon
    //             if is_rectangle_within_polygon(&points[l], &points[r], &squares) {
    //                 p1 = l;
    //                 p2 = r;
    //                 max_area = area;
    //                 println!(
    //                     "New largest constrained area found: {}:{:?} * {}:{:?} = {}",
    //                     l, points[l], r, points[r], area
    //                 );
    //             }
    //         }
    //     }
    // }

    draw_rectangle(points, p1, p2, squares);

    max_area
}

fn is_rectangle_within_polygon(p1: &Point, p2: &Point, squares: &Vec<Vec<Square>>) -> bool {
    for x in p1.map_x.min(p2.map_x)..=p1.map_x.max(p2.map_x) {
        for y in p1.map_y.min(p2.map_y)..=p1.map_y.max(p2.map_y) {
            if squares[y][x] == Square::Outside {
                return false;
            }
        }
    }

    true
}

fn draw_rectangle(
    points: &Vec<Point>,
    p1_idx: usize,
    p2_idx: usize,
    squares: &mut Vec<Vec<Square>>,
) {
    let p1 = &points[p1_idx];
    let p2 = &points[p2_idx];
    for x in p1.map_x.min(p2.map_x)..=p1.map_x.max(p2.map_x) {
        for y in p1.map_y.min(p2.map_y)..=p1.map_y.max(p2.map_y) {
            if squares[y][x] != Square::Red && squares[y][x] != Square::Green {
                squares[y][x] = Square::Rectangle;
            }
        }
    }
}

struct Area {
    area: u64,
    p1_idx: usize,
    p2_idx: usize,
}

fn get_areas(points: &Vec<Point>) -> Vec<Area> {
    let count = points.len();
    let mut areas: Vec<Area> = Vec::with_capacity(count * count - 1);

    for l in 0..count {
        for r in 0..count {
            if l != r {
                let area = points[l].area(&points[r]);
                areas.push(Area {
                    area,
                    p1_idx: l,
                    p2_idx: r,
                });
            }
        }
    }

    areas.sort_by(|a, b| b.area.cmp(&a.area));

    areas
}

/// Brute force gets the largest area from any combination of points
fn largest_area(points: &Vec<Point>) -> u64 {
    let mut max_area: u64 = 0;
    let count = points.len();

    for l in 0..count {
        for r in 0..count {
            let area = points[l].area(&points[r]);
            if area > max_area {
                println!(
                    "New largest area found: {}:{:?} * {}:{:?} = {}",
                    l, points[l], r, points[r], area
                );
                max_area = area;
            }
        }
    }

    max_area
}

fn populate_map<'a>(lines: &mut Vec<Line<'a>>, xlen: usize, ylen: usize) -> Vec<Vec<Square>> {
    let mut squares: Vec<Vec<Square>> = vec![vec![Square::Unknown; xlen]; ylen];

    // Fill in the squares with red and green tiles from the points
    for line in lines {
        // Set the point's squares as red
        squares[line.p1.map_y][line.p1.map_x] = Square::Red;
        squares[line.p2.map_y][line.p2.map_x] = Square::Red;
        // println!(
        //     "Setiting ({}, {}) and ({}, {}) Red",
        //     line.p1.map_x, line.p1.map_y, line.p2.map_x, line.p2.map_y
        // );

        // Set all the squares on the line between as green
        for point in line {
            // println!("Setiting ({}, {}) Green", point.0, point.1);
            squares[point.1][point.0] = Square::Green;
        }
    }

    // Fill in the inside and outside squares
    for x in 0..xlen {
        for y in 0..ylen {
            populate_point_interior(&mut squares, x, y, xlen, ylen);
        }
    }

    squares
}

/// Evaluates whether an unknown point in the polygon is inside or outside the polygon
/// Does nothing for points that are already known
fn populate_point_interior(
    squares: &mut Vec<Vec<Square>>,
    x: usize,
    y: usize,
    xlen: usize,
    ylen: usize,
) {
    if squares[y][x] == Square::Unknown {
        // Extend a line in all four directions from the square until you hit the end
        // If all four arrive at a line segment, you are within the shape
        // Short circuit if any point fails the check

        // Note: for a shape with holes, extend to the end and count the number of intersections
        // and a point is inside if it has an odd number in each direction

        let mut inside = false;

        // North
        for j in (0..y).rev() {
            // println!("North check square[{}][{}] ({:?})", j, x, squares[j][x]);
            match squares[j][x] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    inside = true;
                    // println!("squares[{}][{}] north hit a border", y, x);
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
                    squares[y][x] = Square::Outside;
                    // println!("squares[{}][{}] north found outside", y, x);
                    return;
                }
                Square::Unknown | Square::Rectangle => {
                    // Keep going
                }
            }
        }

        if inside == false {
            // You hit the bounds on the last one, this is outside
            squares[y][x] = Square::Outside;
            // println!("squares[{}][{}] north ran out of graph", y, x);
            return;
        }

        // South
        inside = false;
        for j in y..ylen {
            // println!("South check square[{}][{}] ({:?})", j, x, squares[j][x]);
            match squares[j][x] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    inside = true;
                    // println!("squares[{}][{}] south hit a border", y, x);
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
                    // println!("squares[{}][{}] south found outside", y, x);
                    squares[y][x] = Square::Outside;
                    return;
                }
                Square::Unknown | Square::Rectangle => {
                    // Keep going
                }
            }
        }
        if inside == false {
            // You hit the bounds on the last one, this is outside
            squares[y][x] = Square::Outside;
            return;
        }

        // West
        for i in (0..x).rev() {
            // println!("West check square[{}][{}] ({:?})", y, i, squares[y][i]);
            match squares[y][i] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    // println!("squares[{}][{}] west hit a border", y, x);
                    inside = true;
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
                    // println!("squares[{}][{}] west found outside", y, x);
                    squares[y][x] = Square::Outside;
                    return;
                }
                Square::Unknown | Square::Rectangle => {
                    // Keep going
                }
            }
        }

        if inside == false {
            // You hit the bounds on the last one, this is outside
            squares[y][x] = Square::Outside;
            return;
        }

        // East
        inside = false;
        for i in x..xlen {
            // println!("East check square[{}][{}] ({:?})", y, i, squares[y][i]);
            match squares[y][i] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    inside = true;
                    // println!("squares[{}][{}] east hit a border", y, x);
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
                    // println!("squares[{}][{}] east found outside", y, x);
                    squares[y][x] = Square::Outside;
                    return;
                }
                Square::Unknown | Square::Rectangle => {
                    // Keep going
                }
            }
        }
        if inside == false {
            // You hit the bounds on the last one, this is outside
            squares[y][x] = Square::Outside;
            return;
        }

        // If you've made it this far, you've passed all inside tests!
        // println!("Square[{}][{}] is inside!", y, x);
        squares[y][x] = Square::Inside;
    }
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
    let mut x_map: Vec<i64> = Vec::with_capacity(count / 2 + count / 8);
    let mut y_map: Vec<i64> = Vec::with_capacity(count / 2 + count / 8);

    let mut points_sorted: Vec<Point> = points.clone();

    // Map X coordinates to condensed point space
    points_sorted.sort_by(|a, b| a.x.cmp(&b.x));
    for point_x in &points_sorted {
        if let Some(&last_x) = x_map.get(x_map.len().saturating_sub(1))
            && point_x.x == last_x
        {
            // Use the existing last x_map point, so do nothing here
        } else {
            // Append a new x_map from this point
            x_map.push(point_x.x);
        }
    }

    // Map Y coordinates to condensed point space
    points_sorted.sort_by(|a, b| a.y.cmp(&b.y));
    for point_y in points_sorted {
        if let Some(&last_y) = y_map.get(y_map.len().saturating_sub(1))
            && point_y.y == last_y
        {
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
    // println!("Squares: {} x {}", squares.len(), squares[0].len());
    for line in squares {
        for square in line {
            print!("{}", square);
        }
        println!();
    }
}
