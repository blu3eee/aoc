use std::{ fs::File, collections::HashMap, cmp::Ordering, io::Write };

use utils::{ read_input, create_output_file, sort::quicksort };

fn normalize_data(input: Vec<String>) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|line|
            line
                .trim()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        )
        .collect()
}

fn find_prediction_forward(history: Vec<i64>) -> i64 {
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    sequences.push(history.clone());
    let mut current_difference = history.clone();
    while
        current_difference
            .clone()
            .iter()
            .filter_map(|x| if *x == 0 { Some(1) } else { None })
            .collect::<Vec<i64>>()
            .len() != current_difference.len()
    {
        let mut new_difference: Vec<i64> = Vec::new();
        for i in 1..current_difference.len() {
            new_difference.push(current_difference[i] - current_difference[i - 1]);
        }
        current_difference = new_difference;
        sequences.push(current_difference.clone());
    }
    let total = sequences
        .iter()
        .map(|x| x.last().unwrap())
        .sum::<i64>();
    total
}

fn part1(input: Vec<String>, mut output_file: &File) {
    let total = normalize_data(input)
        .iter()
        .map(|history| find_prediction_forward(history.clone()))
        .sum::<i64>();

    println!("total {total}")
}

// fn find_prediction_backward(history: Vec<i64>) -> i64 {
//     let mut sequences: Vec<Vec<i64>> = Vec::new();
//     sequences.push(history.clone());
//     let mut current_difference = history..clone();

//     0
// }

fn part2(input: Vec<String>, mut output_file: &File) {
    let total = normalize_data(input)
        .iter()
        .map(|history| {
            let mut history = history.clone();
            history.reverse();
            find_prediction_forward(history.clone())
        })
        .sum::<i64>();

    println!("total {total}")
}

fn main() {
    let input = read_input(9);

    let output_file = create_output_file(9);

    part1(input.clone(), &output_file);
    part2(input.clone(), &output_file);
}
