use std::collections::HashMap;

use utils::read_input;

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<Vec<usize>> {
    input
        .split("\n")
        .map(|line|
            line
                .as_bytes()
                .iter()
                .map(|c| if (*c as char) == '.' { 0 } else if (*c as char) == '#' { 1 } else { 2 })
                .collect()
        )
        .collect()
}

// assume this map is in a non-rotated state
fn calculate_total_weight(map: &Vec<Vec<usize>>) -> usize {
    let mut total = 0;
    for line in rotate(map.to_vec()) {
        total += line
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == 2 { Some(i + 1) } else { None })
            .sum::<usize>();
    }
    total
}

fn rotate(map: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut rotated_map: Vec<Vec<usize>> = Vec::new();
    for i in 0..map[0].len() {
        let mut line: Vec<usize> = map
            .iter()
            .map(|x| x[i])
            .collect();
        line.reverse();
        rotated_map.push(line);
    }
    rotated_map
}

// tilt east (left)
fn tilt_left(map: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut tilted_map = Vec::new();
    for line in map {
        let mut sections: Vec<usize> = Vec::new();
        for i in 0..line.len() {
            if line[i] == 1 {
                sections.push(i);
            }
        }

        let mut prev_split = 0;
        let mut new_line = line.clone();
        for i in 0..new_line.len() {
            if new_line[i] == 2 {
                new_line[i] = 0;
            }
        }
        for split in sections.clone() {
            let count = line[prev_split..split]
                .to_vec()
                .iter()
                .filter_map(|&x| if x == 2 { Some(1) } else { None })
                .count();
            for i in split - count..split {
                new_line[i] = 2;
            }

            prev_split = split.clone();
        }
        if !sections.contains(&(line.len() - 1)) {
            let count = line[prev_split..line.len()]
                .to_vec()
                .iter()
                .filter_map(|&x| if x == 2 { Some(1) } else { None })
                .count();
            for i in line.len() - count..line.len() {
                new_line[i] = 2;
            }
        }
        tilted_map.push(new_line);
    }

    tilted_map
}

fn print_map(map: &Vec<Vec<usize>>) {
    for line in map {
        let line = line
            .iter()
            .map(|&x| if x == 2 { "O" } else if x == 1 { "#" } else { "." })
            .collect::<Vec<&str>>()
            .join("");
        println!("{line}");
    }

    println!()
}

fn part1(input: String) -> usize {
    let map = rotate(rotate(rotate(tilt_left(rotate(normalize_data(input))))));

    calculate_total_weight(&map)
}

fn part2(input: String) -> usize {
    let mut current_state: Vec<Vec<usize>> = normalize_data(input);
    let mut memoized: HashMap<Vec<Vec<usize>>, Vec<Vec<usize>>> = HashMap::new();
    for _ in 0..1000 {
        let prev_state = current_state.clone();
        if let Some(state) = memoized.get(&prev_state) {
            // println!("memoizization found - weight {}", calculate_total_weight(&state));
            current_state = state.clone();
        } else {
            for _ in 0..4 {
                current_state = tilt_left(rotate(current_state));
            }
            memoized.entry(prev_state).or_insert(current_state.clone());
        }
    }

    calculate_total_weight(&current_state)
}

fn main() {
    let input = read_input(14);
    println!("{}", part1(input.clone().join("\n")));
    println!("{}", part2(input.clone().join("\n")));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(
            part1(
                r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#.to_string()
            ),
            136
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(
                r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#.to_string()
            ),
            64
        );
    }
}
