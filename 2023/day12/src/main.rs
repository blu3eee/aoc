use std::{ fs::File, collections::HashMap, cmp::Ordering, io::Write, clone };

use utils::{ read_input, create_output_file, sort::quicksort };

fn combinations<T: Clone>(vec: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return vec![Vec::new()];
    }

    if vec.len() < size {
        return Vec::new();
    }

    let mut result = Vec::new();

    for i in 0..vec.len() {
        let item = &vec[i];
        let rest = &vec[i + 1..];

        for mut smaller in combinations(rest, size - 1) {
            let mut next = vec![item.clone()];
            next.append(&mut smaller);
            result.push(next);
        }
    }

    result
}

// part 1
// normalize data and expand the map
fn normalize_data(input: String) -> Vec<(String, Vec<usize>)> {
    input
        .split("\n")
        .map(|x| {
            let parts = x
                .trim()
                .to_string()
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let groups = parts[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (parts[0].clone(), groups)
        })
        .collect()
}

fn part1(input: String) -> usize {
    let mut total_arrangements: usize = 0;
    for (data, groups) in normalize_data(input) {
        // total of # sighs in the data
        let total = groups.iter().sum::<usize>();
        // find the positions of ? signs
        let need_to_fills = data
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if (*c as char) == '?' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        // number of # to find
        let damages_to_fill =
            total -
            data
                .as_bytes()
                .iter()
                .filter_map(|x| if (*x as char) == '#' { Some(1) } else { None })
                .count();
        println!("data {data}");
        total_arrangements += dp_p1(data, groups, need_to_fills, damages_to_fill);
        // println!("");
    }
    total_arrangements
}

fn dp_p1(
    data: String,
    valid_arrangement_groups: Vec<usize>,
    need_to_fills: Vec<usize>,
    damages_to_fill: usize
) -> usize {
    let combs = combinations(&need_to_fills, damages_to_fill);
    let mut total: usize = 0;
    for comb in combs {
        let mut data = data.replace("?", ".");
        for switch in comb {
            data.replace_range(switch..switch + 1, "#");
        }
        let groups = data
            .split(".")
            .filter_map(|x| if x.to_string().len() > 0 { Some(x.to_string().len()) } else { None })
            .collect::<Vec<usize>>();
        if groups == valid_arrangement_groups {
            total += 1;
        }
    }

    total
}

// part 2
fn normalize_data_part2(input: String) -> Vec<(String, Vec<usize>)> {
    input
        .split("\n")
        .map(|x| {
            let parts = x
                .trim()
                .to_string()
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let groups = parts[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let record = parts[0].clone();
            (record, groups)
        })
        .collect()
}

fn part2(input: String) -> usize {
    let mut total_arrangements: usize = 0;
    for (data, groups) in normalize_data_part2(input) {
        println!("data {data} groups {groups:?}");
        let once = check(data.clone(), groups.clone(), 1);
        let twice = check(data.clone(), groups.clone(), 2);

        let multiplier = twice / once;
        total_arrangements += once * multiplier * multiplier * multiplier * multiplier;
    }

    total_arrangements
}

fn check(data: String, groups: Vec<usize>, repeat: usize) -> usize {
    if repeat == 1 {
        println!("check {data} - {groups:?}");
        let total = groups.iter().sum::<usize>();
        // find the positions of ? signs
        let need_to_fills = data
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if (*c as char) == '?' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        // number of # to find
        let damages_to_fill =
            total -
            data
                .as_bytes()
                .iter()
                .filter_map(|x| if (*x as char) == '#' { Some(1) } else { None })
                .count();
        dp_p1(data, groups, need_to_fills, damages_to_fill)
    } else {
        let check_once = check(data.clone(), groups.clone(), 1);
        let new_data = vec![data.clone(); repeat].join("?");
        let new_groups = groups.repeat(repeat);
        let total = new_groups.iter().sum::<usize>();

        // find the positions of ? signs
        let need_to_fills = new_data
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if (*c as char) == '?' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        // number of # to find
        let damages_to_fill =
            total -
            new_data
                .as_bytes()
                .iter()
                .filter_map(|x| if (*x as char) == '#' { Some(1) } else { None })
                .count();
        let combs: Vec<Vec<usize>> = combinations(&need_to_fills, damages_to_fill);
        println!("combs length {}", combs.len());
        let combs: Vec<Vec<usize>> = combs
            .iter()
            .filter_map(|x| if x.contains(&data.len()) { Some(x.clone()) } else { None })
            .collect();
        println!("combs length {}", combs.len());

        println!("check {new_data} - {new_groups:?}");

        let mut total: usize = 0;

        for comb in combs {
            let mut local_data = new_data.replace("?", ".");
            for switch in comb {
                local_data.replace_range(switch..switch + 1, "#");
            }
            let local_groups = local_data
                .split(".")
                .filter_map(|x| (
                    if x.to_string().len() > 0 {
                        Some(x.to_string().len())
                    } else {
                        None
                    }
                ))
                .collect::<Vec<usize>>();
            if local_groups == new_groups {
                total += 1;
            }
        }

        check_once * check_once + total
    }
}

fn main() {
    let input = read_input(12);
    // println!("{}", part1(input.clone().join("\n")));
    println!("{}", part2(input.clone().join("\n")));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(
            part1(
                r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#.to_string()
            ),
            21
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(
                r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#.to_string()
            ),
            525152
        );
    }
}
