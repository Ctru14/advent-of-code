pub(crate) fn solve_day8() {
    // Get the input: list of 3D coordinates of junction boxes to connect
    let (input, number) = (include_str!("day8-input.txt"), 1000);
    // let (input, number) = (include_str!("day8-test.txt"), 10);

    let mut boxes: Vec<JunctionBox> = parse_input(input);
    // print_boxes(&boxes);

    let connections: Vec<Connection> = form_connections(&boxes);
    // print_connections(&connections);

    let circuits = track_circuits(&mut boxes, &connections, number);
    // println!("Circuits: {:?}", circuits);

    evaluate_circuits(&circuits, boxes.len());
}

#[derive(Debug)]
struct JunctionBox {
    // id: usize, // ID (initial index) of this box
    x: i64,
    y: i64,
    z: i64,
    // shortest_id: usize,          // ID of the shortest connection to this box
    // dist: u64,                   // Distance to the shortest ID
    // connected_to: Option<usize>, // ID of the final connection made
    circuit_num: Option<usize>, // Number of the circuit they're a part of
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Connection {
    dist: u64,
    box1_idx: usize,
    box2_idx: usize,
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl JunctionBox {
    fn get_distance_squared_between(&self, other: &JunctionBox) -> u64 {
        (self.x - other.x).pow(2) as u64
            + (self.y - other.y).pow(2) as u64
            + (self.z - other.z).pow(2) as u64
    }
}

/// Form circuits from connected boxes
fn track_circuits(
    boxes: &mut Vec<JunctionBox>,
    connections: &Vec<Connection>,
    target_connections: usize,
) -> Vec<Vec<usize>> {
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut num_circuits: usize = 0;
    let mut connections_made: usize = 0;
    let mut circuits_after_target: Vec<Vec<usize>> = Vec::new();
    let mut num_active_circuits = 0;
    let mut one_circuit = false;

    'outer: for (idx, connection) in connections.iter().enumerate() {
        let b1_idx = connection.box1_idx;
        let b2_idx = connection.box2_idx;
        println!(
            "\nConnection #{}: Boxes {}:{:?} and {}:{:?} (dist {})",
            idx, b1_idx, boxes[b1_idx], b2_idx, boxes[b2_idx], connection.dist
        );

        match (boxes[b1_idx].circuit_num, boxes[b2_idx].circuit_num) {
            (None, None) => {
                // Neither box has a connection. Add a new circuit
                circuits.push(vec![b1_idx, b2_idx]);
                boxes[b1_idx].circuit_num = Some(num_circuits);
                boxes[b2_idx].circuit_num = Some(num_circuits);
                // println!(
                //     "New circuit ID {} has IDs {:?}",
                //     num_circuits, &circuits[num_circuits]
                // );
                num_circuits += 1;
                num_active_circuits += 1;
                connections_made += 1;
            }
            (None, Some(c2)) => {
                // Add box1 to box2's circuit
                boxes[b1_idx].circuit_num = Some(c2);
                circuits[c2].push(b1_idx);
                // println!(
                //     "Adding box {} to {}'s circuit {}: {:?}",
                //     b1_idx, b2_idx, c2, circuits[c2]
                // );
                connections_made += 1;
            }
            (Some(c1), None) => {
                // Add box2 to box1's circuit
                boxes[b2_idx].circuit_num = Some(c1);
                circuits[c1].push(b2_idx);
                // println!(
                //     "Adding box {} to {}'s circuit {}: {:?}",
                //     b2_idx, b1_idx, c1, circuits[c1]
                // );
                connections_made += 1;
            }
            (Some(c1), Some(c2)) => {
                // Both boxes have an existing circuit. Merge if not already
                if c1 != c2 {
                    // println!("Merging circuit {} into circuit {}", c2, c1);
                    for &id in &circuits[c2] {
                        boxes[id].circuit_num = Some(c1);
                    }
                    let mut temp: Vec<usize> = Vec::with_capacity(circuits[c2].len()); // Borrow checker
                    temp.append(&mut circuits[c2]);
                    circuits[c1].append(&mut temp);
                    // println!(
                    //     "Circuits[{}] (len {}): {:?}",
                    //     c1,
                    //     circuits[c1].len(),
                    //     circuits[c1]
                    // );
                    num_active_circuits -= 1;
                    connections_made += 1;
                } else {
                    // Do nothing
                    // println!(
                    //     "Boxes {} and {} are already in circuit {}",
                    //     b1_idx, b2_idx, c1
                    // );
                    connections_made += 1;
                }
            }
        }

        // print!("{}: ", idx);
        if circuits.len() >= 3 {
            evaluate_circuits(&circuits, boxes.len());
        }
        // println!("Connections Made: {}", connections_made);

        if connections_made == target_connections {
            println!("End of connecting!\n\n");
            circuits_after_target = circuits.clone();
        }

        // Check for the first time they make one large circuit
        // if connections_made > target_connections && num_active_circuits == 1 {
        //     println!("\n -------\nOne circuit remains!");
        //     println!("Final boxes connected: {:?} and {:?}", boxes[b1_idx], boxes[b2_idx]);
        //     println!("Product of x coords: {} * {} = {}", boxes[b1_idx].x, boxes[b2_idx].x, boxes[b1_idx].x * boxes[b2_idx].x);
        //     println!("-------");
        //     break;
        // }
    }

    //
    println!("Evaluating circuits after the target:");
    evaluate_circuits(&circuits_after_target, boxes.len());

    circuits
}

fn evaluate_circuits(circuits: &Vec<Vec<usize>>, total: usize) {
    let count = circuits.len();
    println!("Circuits: {:?}", circuits);

    if count >= 3 {
        // Get multiplied number of connections from the three largest circuits
        let mut num_connections_sorted: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
        let sum: usize = num_connections_sorted.iter().sum();
        num_connections_sorted.sort();
        println!(
            "Num circuit connections ({}): {:?}",
            count, num_connections_sorted
        );
        println!("Sum of connections: {}", sum);
        let mult_three = num_connections_sorted[count - 3]
            * num_connections_sorted[count - 2]
            * num_connections_sorted[count - 1];

        println!(
            "Top three circuit sizes: {} * {} * {} = {}",
            num_connections_sorted[count - 3],
            num_connections_sorted[count - 2],
            num_connections_sorted[count - 1],
            mult_three
        );

        if num_connections_sorted[count - 1] == total {
            panic!("Found last pair!");
        }
    }
}

//fn get_idx_by_id(boxes: &Vec<JunctionBox>, id: usize) -> Option<usize> {
//    boxes.iter().position(|b| b.id == id)
//}

/// Boxes should already be sorted by connection distance
fn form_connections<'a>(boxes: &'a Vec<JunctionBox>) -> Vec<Connection> {
    let count = boxes.len();
    let mut connections: Vec<Connection> = Vec::with_capacity(count * count / 2);
    for i in 0..count {
        for j in i..count {
            if i != j {
                let dist_sqr = boxes[i].get_distance_squared_between(&boxes[j]);
                connections.push(Connection {
                    dist: dist_sqr,
                    box1_idx: i,
                    box2_idx: j,
                });
            }
        }
    }

    // assert_eq!(connections.len(), count * count / 2);

    connections.sort();

    connections
}

/*
/// Go through the list of boxes, finding the shortest connection to each one
/// Returns the number of ciruits
fn _populate_shortest_connections<'a>(boxes: &'a mut Vec<JunctionBox>) {
    for current_idx in 0..boxes.len() {
        let mut connected_to_id = boxes[current_idx].id;
        assert_eq!(current_idx, boxes[current_idx].id); // At this point, these should equal
        let mut connected_dist = u64::MAX;

        // Get shortest distance and connection for this box
        for check_idx in 0..boxes.len() {
            // Do not evaluate box on itself
            if current_idx != check_idx {
                let dist_sqr = boxes[current_idx].get_distance_squared_between(&boxes[check_idx]);
                if dist_sqr < connected_dist {
                    connected_dist = dist_sqr;
                    connected_to_id = boxes[check_idx].id;
                    assert_eq!(check_idx, boxes[check_idx].id); // At this point, these should equal
                }
            }
        }

        // Update current box with the connection and assign circuit num
        boxes[current_idx].shortest_id = connected_to_id;
        boxes[current_idx].dist = connected_dist;
        println!(
            "{}: {:?} is closest to {}:{:?} with dist2 {}",
            current_idx,
            boxes[current_idx],
            connected_to_id,
            boxes[connected_to_id],
            connected_dist,
        );
    }
} */

fn parse_input(input: &str) -> Vec<JunctionBox> {
    let mut out: Vec<JunctionBox> = Vec::with_capacity(1000);
    let mut idx: usize = 0;

    for line in input.trim().lines() {
        // Parse x,y,z coordinates from input
        let split: Vec<&str> = line.split(',').collect();
        out.push(JunctionBox {
            // id: idx,
            x: i64::from_str_radix(split[0], 10).unwrap(),
            y: i64::from_str_radix(split[1], 10).unwrap(),
            z: i64::from_str_radix(split[2], 10).unwrap(),
            // shortest_id: idx,
            // dist: u64::MAX,
            // connected_to: None,
            circuit_num: None,
        });
        idx += 1;
    }

    out
}

fn print_boxes(boxes: &Vec<JunctionBox>) {
    for jbox in boxes {
        println!("{:?}", jbox);
    }
}

fn print_connections(connections: &Vec<Connection>) {
    for connection in connections {
        println!("{:?}", connection);
    }
}
