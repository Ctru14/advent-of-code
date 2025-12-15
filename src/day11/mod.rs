use std::collections::HashMap;

pub(crate) fn solve_day11() {
    // Get the input: data paths from a server
    // let input = include_str!("day11-input.txt");
    let input = include_str!("day11-test.txt");

    let servers = construct_servers(input);

    print_servers(&servers);
}

#[derive(Debug)]
struct Server<'a> {
    name: &'a str,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl<'a> From<&'a str> for Server<'a> {
    fn from(value: &'a str) -> Self {
        // ccc: ddd eee fff
        // Every name is always three characters
        let outputs: Vec<&str> = value[5..].trim().split(' ').collect();
        Self {
            name: &value[0..3],
            inputs: Vec::with_capacity(outputs.len() * 2),
            outputs,
        }
    }
}

/// Construct a HashMap of servers from the given lines
/// Populates the inputs of a server based on the output of
/// the current server being parsed
fn construct_servers<'a>(input: &'a str) -> HashMap<&'a str, Server<'a>> {
    let mut map: HashMap<&str, Server<'a>> = HashMap::new();

    for line in input.lines() {
        // Create server object
        let server = Server::from(line);

        // Populate this server's outputs as other server's inputs
        for &output in &server.outputs {
            if let Some(output_existing) = map.get_mut(output) {
                output_existing.inputs.push(server.name);
            } else {
                // Create new server with this name and this as an input
                map.insert(
                    output,
                    Server {
                        name: output,
                        inputs: vec![server.name],
                        outputs: Vec::new(),
                    },
                );
            }
        }

        // See if the server already exists
        if let Some(this_existing) = map.get_mut(server.name) {
            // Populate outputs to existing entry
            this_existing.outputs = server.outputs;
        } else {
            // Insert new entry to the map
            map.insert(server.name, server);
        }
    }

    map
}

fn print_servers(servers: &HashMap<&str, Server>) {
    println!("Servers ({}):", servers.len());
    for server in servers {
        println!("{:?}", server);
    }
}
