use std::collections::HashMap;

pub(crate) fn solve_day11() {
    // Get the input: data paths from a server
    let input = include_str!("day11-input.txt");
    // let input = include_str!("day11-test-p1.txt");
    // let input = include_str!("day11-test-p2.txt");

    let servers = construct_servers(input);
    // print_servers(&servers);

    // P1: Paths from you to out
    let mut cache: HashMap<&str, usize> = HashMap::new();
    let you_out_paths = how_many_paths_between("you", "out", &servers, &mut cache);
    println!("P1: There are {} paths from 'you' to 'out'", you_out_paths);

    // P2: Paths from svr to out that contain both fft and dac
    let svr_dac = how_many_paths_between("svr", "dac", &servers, &mut HashMap::new());
    let svr_fft = how_many_paths_between("svr", "fft", &servers, &mut HashMap::new());
    let dac_fft = how_many_paths_between("dac", "fft", &servers, &mut HashMap::new());
    let fft_dac = how_many_paths_between("fft", "dac", &servers, &mut HashMap::new());
    let dac_out = how_many_paths_between("dac", "out", &servers, &mut HashMap::new());
    let fft_out = how_many_paths_between("fft", "out", &servers, &mut HashMap::new());

    println!(
        "P2: Paths between:\nsvr-dac: {}\nsvr-fft: {}\ndac-fft: {}\nfft-dac: {}\ndac-out: {}\nfft-out: {}",
        svr_dac, svr_fft, dac_fft, fft_dac, dac_out, fft_out
    );

    // Logic to find number of paths:
    // Since there are no loops in this graph, exactly one of fft-dac and dac-fft paths will be zero
    let (svr_first, first_second, second_out) = match (dac_fft, fft_dac) {
        (0, _) => {
            // FFT comes before DAC
            (svr_fft, fft_dac, dac_out)
        }
        (_, 0) => {
            // DAC comes before FFT
            (svr_dac, dac_fft, fft_out)
        }
        _ => {
            panic!("AAAAHHHH");
        }
    };

    println!("\nsvr-first: {}\nfirst-second: {}\nsecond-out: {}", svr_first, first_second, second_out);
    let total = svr_first * first_second * second_out;
    println!("\nTotal paths: {}", total);
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

/// Counts the paths from one server to another
fn how_many_paths_between<'a>(
    from: &'a str, // Server,
    to: &'a str,   // Server,
    servers: &'a HashMap<&'a str, Server<'a>>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if from == to {
        // Ending case
        return 1;
    }
    if let Some(&val) = cache.get(from) {
        return val;
    }
    let from_node = servers.get(from).unwrap();
    let val: usize = from_node
        .outputs
        .iter()
        .map(|&node| how_many_paths_between(node, to, servers, cache))
        .sum();
    cache.insert(from, val);
    val
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
