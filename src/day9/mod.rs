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

    // Tests
    // test_points(&points, &x_map, &y_map);
    // test_map_monotonic(&x_map);
    // test_map_monotonic(&y_map);

    let mut squares = populate_map(&mut lines, x_map.len(), y_map.len());

    println!("\nx_map (len {}): {:?}", x_map.len(), x_map);
    println!("\ny_map (len {}): {:?}", y_map.len(), y_map);

    // println!("\nPoints: {:?}", points);
    // println!("\nLines: {:?}", lines);

    let (idx, max_constrained_area) =
        largest_constrained_area(&areas, &points, &mut squares, &lines);

    // Draw the rectangle
    println!("Showing area[{}]", idx);
    draw_rectangle(
        &points[areas[idx].p1_idx],
        &points[areas[idx].p2_idx],
        &mut squares,
    );

    print_squares(&squares);
    // Checking area[103606]: 1429043625 is too low
    // Checking area[102000]: 1462492632 is too high
    // Checking area[94761]: 1621520882 is too high

    println!("Largest constrained area: {}", max_constrained_area);
    // 1429075575 is just right!
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
            Square::Outside => write!(f, " "),
            Square::Unknown => write!(f, "?"),
            Square::Rectangle => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
    map_x: usize,
    map_y: usize,
}

impl Point {
    fn area(&self, other: &Point) -> u64 {
        (self.x.max(other.x) - self.x.min(other.x) + 1)
            * (self.y.max(other.y) - self.y.min(other.y) + 1)
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.trim().split(',').collect();
        Self {
            x: u64::from_str_radix(split[0], 10).unwrap(),
            y: u64::from_str_radix(split[1], 10).unwrap(),
            map_x: 0, // Will fill in later
            map_y: 0, // Will fill in later
        }
    }
}

#[derive(Debug)]
struct Line<'a> {
    start: &'a Point,
    end: &'a Point,
    current: (usize, usize),
    update: (usize, usize),
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
        // Lines cannot be diagonal
        assert!(p1.x == p2.x || p1.y == p2.y);

        // Since one coordinate is the same, we need to determine which one's map coordinate is higher
        let update_test = (
            (p1.map_x as i64 - p2.map_x as i64).signum(),
            (p1.map_y as i64 - p2.map_y as i64).signum(),
        );
        let (current, start, end): ((usize, usize), &Point, &Point) =
            if update_test.0 < 0 || update_test.1 < 0 {
                // P2 has a higher coordinate - use P1
                ((p1.map_x, p1.map_y), p1, p2)
            } else {
                // P1 has a higher coordinate - use P2
                ((p2.map_x, p2.map_y), p2, p1)
            };
        let update = (update_test.0.abs() as usize, update_test.1.abs() as usize);

        Self {
            start,
            end,
            current,
            update,
        }
    }
}

fn largest_constrained_area(
    areas: &Vec<Area>,
    points: &Vec<Point>,
    squares: &mut Vec<Vec<Square>>,
    lines: &Vec<Line>,
) -> (usize, u64) {
    let mut max_area = 0;
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut idx = 0;

    for (i, area) in areas.iter().enumerate() {
        // println!("Checking area[{}]: {}", i, area.area);
        if is_rectangle_within_polygon(&points[area.p1_idx], &points[area.p2_idx], &squares, lines)
        {
            p1 = area.p1_idx;
            p2 = area.p2_idx;
            max_area = area.area;
            idx = i;
            println!(
                "Largest constrained area found: {}:{:?} * {}:{:?} = {}",
                p1, points[area.p1_idx], p2, points[area.p2_idx], area.area
            );
            break;
        }
    }

    // let count = points.len();
    // for l in 0..count {
    //     for r in 0..count {
    //         let area = points[l].area(&points[r]);
    //         if area > max_area {
    //             // Check if all the points in this rectangle are within the polygon
    //             if is_rectangle_within_polygon(&points[l], &points[r], &squares, lines) {
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

    (idx, max_area)
}

fn is_rectangle_within_polygon(
    p1: &Point,
    p2: &Point,
    squares: &Vec<Vec<Square>>,
    lines: &Vec<Line>,
) -> bool {
    // Check using the map
    let mut map_check = true;
    for x in p1.map_x.min(p2.map_x)..=p1.map_x.max(p2.map_x) {
        for y in p1.map_y.min(p2.map_y)..=p1.map_y.max(p2.map_y) {
            if squares[y][x] == Square::Outside {
                map_check = false;
                break;
            }
        }
    }

    // let line_check = _line_check_rectangle_within_polygon(p1, p2, squares, lines, map_check);

    return map_check;
}

fn draw_rectangle(
    // points: &Vec<Point>,
    p1: &Point,
    p2: &Point,
    squares: &mut Vec<Vec<Square>>,
) {
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
        for r in 0..l {
            let area = points[l].area(&points[r]);
            assert_eq!(area, points[r].area(&points[l]));
            areas.push(Area {
                area,
                p1_idx: l,
                p2_idx: r,
            });
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
        squares[line.start.map_y][line.start.map_x] = Square::Red;
        squares[line.end.map_y][line.end.map_x] = Square::Red;

        // Set all the squares on the line between as green
        for point in line {
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
            match squares[j][x] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    inside = true;
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
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

        // South
        inside = false;
        for j in y..ylen {
            match squares[j][x] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    inside = true;
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
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
            match squares[y][i] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    inside = true;
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
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
            match squares[y][i] {
                Square::Red | Square::Green | Square::Inside => {
                    // This direction is inside
                    inside = true;
                    break;
                }
                Square::Outside => {
                    // If you hit outside before a border, you're outside
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
fn condense_point_space(points: &mut Vec<Point>) -> (Vec<u64>, Vec<u64>) {
    let count = points.len();
    let mut x_map: Vec<u64> = Vec::with_capacity(count / 2 + count / 8);
    let mut y_map: Vec<u64> = Vec::with_capacity(count / 2 + count / 8);

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
    println!("Squares: {} x {}", squares.len(), squares[0].len());
    for line in squares {
        for square in line {
            print!("{}", square);
        }
        println!();
    }
}

fn test_points(points: &Vec<Point>, x_map: &Vec<u64>, y_map: &Vec<u64>) {
    for point in points {
        assert_eq!(x_map[point.map_x], point.x);
        assert_eq!(y_map[point.map_y], point.y);
    }
    println!("Points pass");
}

fn test_map_monotonic(map: &Vec<u64>) {
    for i in 0..map.len() {
        for j in 0..i {
            assert!(map[i] > map[j]);
        }
    }
}

fn _line_check_rectangle_within_polygon(
    p1: &Point,
    p2: &Point,
    squares: &Vec<Vec<Square>>,
    lines: &Vec<Line>,
    map_check: bool,
) -> bool {
    let mut line_check = true;
    // Check using the lines - if any of the 4 rectangle lie segments intersects another
    let p21 = &Point {
        x: p2.x,
        y: p1.y,
        map_x: p2.map_x,
        map_y: p1.map_y,
    };
    let p12 = &Point {
        x: p1.x,
        y: p2.y,
        map_x: p1.map_x,
        map_y: p2.map_y,
    };
    let rect_lines: [Line; 4] = [
        Line::new(p1, p12),
        Line::new(p12, p2),
        Line::new(p2, p21),
        Line::new(p21, p1),
    ];
    for rect_line in &rect_lines {
        // Check if this line intersects any line in the list
        for line in lines {
            if _do_lines_cross_through(&rect_line, &line) {
                // println!("Lines cross through!\n{:?}\n{:?}", rect_line, line);
                line_check = false;
                break;
            }
        }
    }

    // The line check still has a bug
    if line_check != map_check {
        println!(
            "\nis_rectangle_within_polygon: Line check ({}) and map check ({}) do not line up!",
            line_check, map_check
        );
        println!("{:?}\n{:?}", p1, p2);
        for (i, rect_line) in rect_lines.iter().enumerate() {
            println!("{}: {:?}", i, rect_line);
            // Check if this line intersects any line in the list
            for line in lines {
                if _do_lines_cross_through(&rect_line, &line) {
                    println!("{}: lines cross!\n{:?}\n{:?}", i, rect_line, line);
                }
            }
        }

        let mut draw_squares = squares.clone();
        draw_rectangle(&p1, &p2, &mut draw_squares);
        print_squares(&draw_squares);
    }
    return line_check;
}

/// Returns true if the lines fully cross through each other
///   |          --|---   ---     --=---
/// ------         |        ---
///   |            |        no      no
/// L1: (2,0)--(2,5) L2: (1,2)--(4,2)
fn _do_lines_cross_through(l1: &Line, l2: &Line) -> bool {
    // (59, 65)--(59, 192)
    // (57, 181)--(60, 181)
    // Should be true because:
    //   65 < 181 and 192 > 181 and 59 >= 57 and 59 <= 60
    // println!("l1: {:?}\nl2: {:?}", l1, l2);
    (l1.start.x < l2.start.x // 59 < 57 nope
        && l1.end.x > l2.end.x
        && l1.start.y >= l2.start.y
        && l1.start.y <= l2.end.y)
        || (l1.start.y < l2.start.y // 65 < 181 yep
            && l1.end.y > l2.end.y // 192 > 181 yep
            && l1.start.x >= l2.start.x // 59 >= 60 nope
            && l1.start.x <= l2.end.x) //  59 <= 60 nope
    // Check if l1 is horizontal or vertical
    // match l1.update {
    //     (1, 0) => {
    //         // (0, 1) => {
    //         // Horizontal
    //         l1.start.x < l2.start.x
    //             && l1.end.x > l2.end.x
    //             && l1.start.y >= l2.end.y
    //             && l1.start.y <= l2.end.y
    //     }
    //     (0, 1) => {
    //         // (1, 0) => {
    //         // Vertical
    //         l1.start.y < l2.start.y
    //             && l1.end.y > l2.end.y
    //             && l1.start.x >= l2.end.x
    //             && l1.start.x <= l2.end.x
    //     }
    //     _ => {
    //         panic!("Line must be horizontal or vertical");
    //     }
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_cross_through() {
        let l1 = Line {
            start: &Point {
                x: 59,
                y: 65,
                map_x: 59,
                map_y: 65,
            },
            end: &Point {
                x: 59,
                y: 192,
                map_x: 59,
                map_y: 192,
            },
            current: (0, 0),
            update: (0, 1),
        };
        let l2 = Line {
            start: &Point {
                x: 57,
                y: 181,
                map_x: 57,
                map_y: 181,
            },
            end: &Point {
                x: 60,
                y: 181,
                map_x: 60,
                map_y: 181,
            },
            current: (0, 0),
            update: (1, 0),
        };
        assert!(_do_lines_cross_through(&l1, &l2));

        let l3 = Line {
            start: &Point {
                x: 2,
                y: 0,
                map_x: 2,
                map_y: 0,
            },
            end: &Point {
                x: 2,
                y: 5,
                map_x: 2,
                map_y: 5,
            },
            current: (0, 0),
            update: (0, 1),
        };
        let l4 = Line {
            start: &Point {
                x: 1,
                y: 2,
                map_x: 1,
                map_y: 2,
            },
            end: &Point {
                x: 4,
                y: 2,
                map_x: 4,
                map_y: 2,
            },
            current: (0, 0),
            update: (1, 0),
        };
        assert!(_do_lines_cross_through(&l3, &l4));
    }
}
