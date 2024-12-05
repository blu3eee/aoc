use std::{ fs::File, collections::HashMap, io::Write };

use utils::{ read_input, create_output_file };

fn normalize_game(game_input: &str) -> Vec<(String, usize)> {
    let mut cubes_map: HashMap<String, usize> = HashMap::new();
    let rounds = game_input
        .split_at(game_input.find(":").unwrap() + 1)
        .1.split(";")
        .map(|round| round.trim().to_string())
        .collect::<Vec<String>>();

    for round in rounds {
        let sets = round
            .split(",")
            .map(|set| {
                let parts = set.trim().split(" ").map(String::from).collect::<Vec<String>>();

                (parts[0].trim().parse::<usize>().unwrap(), parts[1].trim().to_string())
            })
            .collect::<Vec<(usize, String)>>();
        for (amount, color) in sets {
            match cubes_map.entry(color) {
                std::collections::hash_map::Entry::Occupied(mut val) => {
                    if val.get() < &amount {
                        val.insert(amount);
                    }
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(amount);
                }
            }
        }
    }

    cubes_map.into_iter().collect()
}

#[allow(dead_code)]
fn part1(input: Vec<String>, mut output_file: File) {
    let limit: HashMap<&str, usize> = HashMap::from_iter(
        vec![("red", 12), ("green", 13), ("blue", 14)]
    );

    let mut valid_games: Vec<usize> = Vec::new();
    for (i, game) in input.iter().enumerate() {
        let game_max_cubes = normalize_game(game);
        println!("{game_max_cubes:?}");
        output_file
            .write_all(&format!("game {}: {:?}", i + 1, game_max_cubes).as_bytes())
            .expect("unable to write to output file");
        let mut valid = true;
        for (color, amount) in game_max_cubes.iter() {
            if amount > limit.get(color.as_str()).unwrap() {
                valid = false;
                break;
            }
        }
        if valid {
            valid_games.push(i + 1);
        }
    }

    println!("{valid_games:?}");
    output_file
        .write_all(&format!("valid games: {:?}", valid_games).as_bytes())
        .expect("unable to write to output file");
    println!("{}", valid_games.iter().sum::<usize>())
}

fn part2(input: Vec<String>, mut output_file: File) {
    let mut sum_of_power = 0;
    for (i, game) in input.iter().enumerate() {
        let game_max_cubes = normalize_game(game);
        println!("{game_max_cubes:?}");
        output_file
            .write_all(&format!("game {}: {:?}", i + 1, game_max_cubes).as_bytes())
            .expect("unable to write to output file");
        let mut set_power: usize = 1;
        for (_, amount) in game_max_cubes.iter() {
            set_power *= amount;
        }
        sum_of_power += set_power;
    }

    output_file
        .write_all(&format!("sum of power: {}", sum_of_power).as_bytes())
        .expect("unable to write to output file");
    println!("{}", sum_of_power)
}

fn main() {
    let day = 2;
    let input = read_input(day);
    let output_file = create_output_file(day);

    part2(input, output_file);
}
