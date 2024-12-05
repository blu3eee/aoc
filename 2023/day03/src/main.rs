use std::{ env, collections::HashMap, io::Write, u64 };
use itertools::Itertools;
use utils::{ read_input, create_output_file };

// 0-9
// .
// !@#$%^

// Check if a character is a symbol
fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}
// ....................983.446.............79......
fn get_number(row: String, idx: usize) -> u64 {
    let (mut l, mut r) = (idx as i64, idx as i64);

    let row = row.as_bytes();
    loop {
        if l >= 0 && (row[l as usize] as char).is_numeric() {
            l -= 1;
        } else {
            break;
        }
    }

    loop {
        if r < (row.len() as i64) && (row[r as usize] as char).is_numeric() {
            r += 1;
        } else {
            break;
        }
    }

    std::str
        ::from_utf8(&row[(l + 1) as usize..=(r - 1) as usize])
        .unwrap()
        .parse::<u64>()
        .unwrap()
}
#[allow(dead_code)]
fn day3_part1(input: Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    let mut file = create_output_file(3);

    // let m = input.len();
    // let n = input[0].len();
    let mut idxes: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..input.len() {
        let row = input[i].as_bytes();
        for j in 0..row.len() {
            let c = row[j] as char;
            if is_symbol(c) {
                // check around
                // check above row
                for i_diff in 0..=2 {
                    for j_diff in 0..=2 {
                        let cur_i = (i as i64) + (i_diff as i64) - 1;
                        let cur_j = (j as i64) + (j_diff as i64) - 1;
                        if
                            cur_i >= 0 &&
                            cur_i <= ((input.len() - 1) as i64) &&
                            cur_j >= 0 &&
                            cur_j <= (row.len() as i64)
                        {
                            if
                                (
                                    input[cur_i as usize].as_bytes()[cur_j as usize] as char
                                ).is_numeric()
                            {
                                match idxes.entry(cur_i as usize) {
                                    std::collections::hash_map::Entry::Occupied(mut val) => {
                                        if !val.get().contains(&(cur_j as usize)) {
                                            val.get_mut().push(cur_j as usize);
                                        }
                                    }
                                    std::collections::hash_map::Entry::Vacant(entry) => {
                                        entry.insert(vec![cur_j as usize]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for (key, values) in idxes.iter().sorted() {
        let mut values = values.clone();
        values.sort();

        // file.write_all(format!("{key}, {values:?}").as_bytes()).expect("error writing file");

        let mut prev: Option<usize> = None;
        let mut nums: Vec<u64> = Vec::new();
        for value in values {
            match prev {
                Some(prev_val) => {
                    if value - prev_val <= 1 {
                        prev = Some(value);
                        continue;
                    }
                }
                None => {}
            }
            prev = Some(value);
            let number = get_number(input[*key].clone(), value);
            nums.push(number);
            sum += number;
        }
        file.write_all(format!("{nums:?}\n").as_bytes()).expect("error writing file");
    }

    sum
}

fn day3_part2(input: Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    let mut file = create_output_file(3);
    for i in 0..input.len() {
        let row = input[i].as_bytes();
        for j in 0..row.len() {
            let c = row[j] as char;
            if is_symbol(c) {
                let mut idxes: HashMap<usize, Vec<usize>> = HashMap::new();
                // check around
                let mut nums: Vec<u64> = Vec::new();
                for i_diff in 0..=2 {
                    for j_diff in 0..=2 {
                        let cur_i = (i as i64) + (i_diff as i64) - 1;
                        let cur_j = (j as i64) + (j_diff as i64) - 1;
                        if
                            cur_i >= 0 &&
                            cur_i <= ((input.len() - 1) as i64) &&
                            cur_j >= 0 &&
                            cur_j <= (row.len() as i64)
                        {
                            let c = input[cur_i as usize].as_bytes()[cur_j as usize] as char;
                            if c.is_numeric() {
                                match idxes.entry(cur_i as usize) {
                                    std::collections::hash_map::Entry::Occupied(mut val) => {
                                        if !val.get().contains(&(cur_j as usize)) {
                                            val.get_mut().push(cur_j as usize);
                                        }
                                    }
                                    std::collections::hash_map::Entry::Vacant(entry) => {
                                        entry.insert(vec![cur_j as usize]);
                                    }
                                }
                            }
                        }
                    }
                }

                for (key, values) in idxes.iter().sorted() {
                    let mut values = values.clone();
                    values.sort();

                    // file.write_all(format!("{key}, {values:?}").as_bytes()).expect("error writing file");

                    let mut prev: Option<usize> = None;

                    for value in values {
                        match prev {
                            Some(prev_val) => {
                                if value - prev_val <= 1 {
                                    prev = Some(value);
                                    continue;
                                }
                            }
                            None => {}
                        }
                        prev = Some(value);
                        let number = get_number(input[*key].clone(), value);
                        nums.push(number);
                    }
                }

                file.write_all(format!("{nums:?}\n").as_bytes()).expect("error writing file");

                if c == '*' {
                    println!("{nums:?}");
                    let mut local = 1;
                    if nums.len() > 1 {
                        for num in nums {
                            local *= num;
                        }
                        sum += local;
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    println!("{}", env::current_dir().unwrap().display());

    println!("{}", day3_part2(read_input(3)));
}
