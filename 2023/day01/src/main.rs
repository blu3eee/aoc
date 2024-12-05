use std::{ collections::HashMap, io::Write };

use utils::{ create_output_file, read_input };

#[allow(dead_code)]
fn part1(input: Vec<String>) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    for line in input {
        let (mut l, mut r) = (0 as usize, line.len() - 1);

        loop {
            if (line.as_bytes()[l] as char).is_numeric() {
                break;
            } else {
                l += 1;
            }
        }

        loop {
            if (line.as_bytes()[r] as char).is_numeric() {
                break;
            } else {
                r -= 1;
            }
        }
        nums.push(
            std::str
                ::from_utf8(&line.as_bytes()[l..l + 1])
                .unwrap()
                .parse::<u32>()
                .unwrap() *
                10 +
                std::str
                    ::from_utf8(&line.as_bytes()[r..r + 1])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
        );
    }
    nums.iter().sum::<u32>()
}
fn part2(input: Vec<String>) -> u32 {
    let mut file = create_output_file(1);
    let numbers: HashMap<&str, u32> = HashMap::from_iter(
        vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ]
    );

    let mut nums: Vec<u32> = Vec::new();
    for line in input {
        file.write_all(format!("line {line:?}\n").as_bytes()).expect("error writing file");
        println!("{line:?}");
        let mut l = line.len();
        let mut r = 0;
        let mut l_num: u32 = 0;
        let mut r_num: u32 = 0;
        for (key, val) in numbers.clone().into_iter() {
            if let Some(idx) = line.find(key) {
                if idx < l {
                    l = idx;
                    l_num = val;
                }
                if idx >= r {
                    r = idx;
                    r_num = val;
                }
            }
            if let Some(idx) = line.rfind(key) {
                if idx < l {
                    l = idx;
                    l_num = val;
                }
                if idx >= r {
                    r = idx;
                    r_num = val;
                }
            }
        }
        // file.write_all(format!("{l_num}{r_num}\n").as_bytes()).expect("error writing file");
        for idx in 0..l {
            if (line.as_bytes()[idx] as char).is_numeric() {
                l = idx;
                l_num = std::str
                    ::from_utf8(&line.as_bytes()[l..l + 1])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                break;
            }
        }
        for idx in 0..line.len() - r {
            let j = line.len() - 1 - idx;
            if (line.as_bytes()[j] as char).is_numeric() {
                r = j;
                r_num = std::str
                    ::from_utf8(&line.as_bytes()[r..r + 1])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                break;
            }
        }
        file.write_all(format!("{l_num}{r_num}\n").as_bytes()).expect("error writing file");
        println!("{l_num}{r_num}");
        nums.push(l_num * 10 + r_num);
    }
    nums.iter().sum::<u32>()
}
fn main() {
    let input = read_input(1);

    println!("{}", part2(input));
}
