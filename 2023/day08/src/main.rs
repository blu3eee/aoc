use std::{ collections::HashMap, io::Write };

use utils::{ create_output_file, read_input, read_test_input };

fn normalize_data(input: Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let content = input.join("\n");
    let parts = content
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let directions = parts[0].clone();

    let network: HashMap<String, (String, String)> = HashMap::from_iter(
        parts[1]
            .split("\n")
            .map(|line| {
                let parts = line
                    .split("=")
                    .map(|x|
                        x.trim().trim_matches('(').trim_matches(')').trim_matches(' ').to_string()
                    )
                    .collect::<Vec<String>>();
                let networks = parts[1]
                    .split(',')
                    .map(|x| x.trim().to_string())
                    .collect::<Vec<String>>();
                (parts[0].clone(), (networks[0].clone(), networks[1].clone()))
            })
            .collect::<Vec<(String, (String, String))>>()
    );

    (directions, network)
}

fn part1(input: Vec<String>) -> usize {
    let (directions, network) = normalize_data(input);
    println!("{network:?}");

    let mut current = "AAA";
    let mut steps: usize = 0;
    while current != "ZZZ" {
        println!("current {current}");
        steps += 1;
        let (left, right) = network.get(current).unwrap();
        println!("({left}, {right})");
        current = if (directions.as_bytes()[(steps - 1) % directions.len()] as char) == 'L' {
            println!("go left");
            left.as_str()
        } else {
            println!("go right");
            right.as_str()
        };
    }
    steps
}

fn part2(input: Vec<String>) -> usize {
    let (directions, network) = normalize_data(input);
    let mut output_file = create_output_file(8);
    let _ = output_file.write_all(&format!("network {network:?}\n").as_bytes());

    let mut starting_nodes: Vec<(String, usize)> = network
        .iter()
        .filter_map(|(x, _)| if x.ends_with('A') { Some(x.to_string()) } else { None })
        .map(|x| (x, 0))
        .collect();

    let mut steps: usize = 0;

    let _ = output_file.write_all(
        &format!("starting node count {}\n", starting_nodes.len()).as_bytes()
    );

    let mut memoized: HashMap<(String, usize), (usize, String)> = HashMap::new();
    let mut paths: Vec<Vec<String>> = vec![Vec::new(); starting_nodes.len()];
    let mut start_indexes: Vec<usize> = vec![0; starting_nodes.len()];
    for (i, (node, _)) in starting_nodes.iter().enumerate() {
        paths[i].push(node.clone().to_string());
    }

    while
        starting_nodes
            .iter()
            .filter_map(|(x, step)| if x.ends_with('Z') && *step == steps { Some(1) } else { None })
            .collect::<Vec<usize>>()
            .len() != starting_nodes.len()
    {
        println!("memoized {memoized:?}");
        // let _ = output_file.write_all(&format!("starting_nodes {starting_nodes:?}\n").as_bytes());
        steps += 1;
        let index = (steps - 1) % directions.len();

        let left_right: usize = if (directions.as_bytes()[index] as char) == 'L' { 0 } else { 1 };

        for (cur_node_index, (current_node, step)) in starting_nodes.clone().iter().enumerate() {
            // println!("current node {} index {}".to_string(), current_node.to_string(), index);
            if step <= &steps {
                // check for memoized hash map
                if
                    let Some((known_step, known_destination)) = memoized.get(
                        &(current_node.to_string(), index)
                    )
                {
                    println!("found memoized");
                    // println!(
                    //     "found memoized node from {current_node} to {known_destination} with {known_step} steps"
                    // );
                    starting_nodes[cur_node_index] = (
                        known_destination.to_string(),
                        step + known_step,
                    );
                    continue;
                }
                let (left, right) = network.get(current_node).unwrap();
                let destination_node = if left_right == 0 {
                    left.to_string()
                } else {
                    right.to_string()
                };
                paths[cur_node_index].push(destination_node.clone());
                if destination_node.ends_with('Z') {
                    for (path_index, path_node) in paths[cur_node_index]
                        .clone()
                        .iter()
                        .enumerate() {
                        let start_index = start_indexes[cur_node_index] + path_index;
                        memoized
                            .entry((path_node.to_string(), start_index))
                            .or_insert((steps - path_index, destination_node.clone()));
                    }
                    start_indexes[cur_node_index] = steps;
                    paths[cur_node_index] = Vec::new();
                }

                starting_nodes[cur_node_index] = (destination_node, step + 1);
            }
        }
    }
    steps
}

fn main() {
    let day_number = 8;
    // part 1
    // run test first
    let test_input = read_test_input(day_number);

    let test_result = part1(test_input.clone());

    let expected_test_output = 2;
    assert_eq!(test_result, expected_test_output);

    let input = read_input(day_number);

    let result = part1(input.clone());
    println!("part1 result {result}");

    // part 2
    // run test first
    // test input for part 2
    /*
    LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)

    */
    let test_input_2: Vec<String> = [
        "LR".to_string(),
        "".to_string(),
        "11A = (11B, XXX)".to_string(),
        "11B = (XXX, 11Z)".to_string(),
        "11Z = (11B, XXX)".to_string(),
        "22A = (22B, XXX)".to_string(),
        "22B = (22C, 22C)".to_string(),
        "22C = (22Z, 22Z)".to_string(),
        "22Z = (22B, 22B)".to_string(),
        "XXX = (XXX, XXX)".to_string(),
    ]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let test_result = part2(test_input_2.clone());

    let expected_test_output = 6;
    assert_eq!(test_result, expected_test_output);

    let result_part2 = part2(input.clone());

    println!("part2 result {result_part2}");
}
