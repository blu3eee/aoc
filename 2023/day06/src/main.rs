use std::fs::File;

use utils::{ read_input, create_output_file };

fn normalize_data(input: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let time = input[0]
        .split_at(input[0].find(":").unwrap() + 1)
        .1.trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distance = input[1]
        .split_at(input[1].find(":").unwrap() + 1)
        .1.trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    (time, distance)
}

fn part1(input: Vec<String>, mut output_file: &File) {
    let (time, distance) = normalize_data(input);

    let mut values: Vec<u64> = Vec::new();

    for i in 0..time.len() {
        let (race_time, record) = (time[i], distance[i]);
        for hold_secs in 0..race_time {
            if hold_secs * (race_time - hold_secs) > record {
                values.push(race_time + 1 - hold_secs * 2);
                break;
            }
        }
    }
    let mut ways = 1;
    for x in values.clone() {
        ways *= x;
    }
    println!("{values:?} {ways}");
}

fn part2(input: Vec<String>, mut output_file: &File) {
    let (time, distance) = normalize_data(input);

    let race_time = time
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u128>()
        .unwrap();

    let record = distance
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u128>()
        .unwrap();

    let mut values: Vec<u64> = Vec::new();
    println!("{time:?} {record}");

    for hold_secs in 0..race_time + 1 {
        if hold_secs * (race_time - hold_secs) > record {
            println!("{}", race_time + 1 - hold_secs * 2);
            break;
        }
    }
}

fn main() {
    let input = read_input(6);

    let output_file = create_output_file(6);

    part1(input.clone(), &output_file);
    part2(input.clone(), &output_file);
    // 10834440
}
