// Start timeL: 12:38 PM Dec 4
// Stop time: 1:48 PM Dev 4
// Time: 1 hour 10 minutes

use std::{ fs::File, io::Write, collections::HashMap };

use utils::{ read_input, create_output_file };

fn normalize_data(line: String) -> (Vec<u32>, Vec<u32>) {
    let cards = line.split_at(line.find(":").unwrap() + 1).1;
    let (win, your) = cards.split_at(cards.find("|").unwrap());
    // println!("{your}");
    (
        win
            .trim()
            .split(" ")
            .filter_map(|each| if each.is_empty() { None } else { Some(each) })
            .map(|num| num.trim().to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
        your
            .trim()[1..]
            .split(" ")
            .filter_map(|each| if each.is_empty() { None } else { Some(each) })
            .map(|num| num.trim().to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
    )
}

#[allow(dead_code)]
fn part1(input: Vec<String>, mut output_file: File) -> u32 {
    let mut total: u32 = 0;
    for (i, line) in input.iter().enumerate() {
        let (win_nums, your_nums) = normalize_data(line.clone());
        let mut count: u32 = 0;
        for num in your_nums {
            if win_nums.contains(&num) {
                count += 1;
            }
        }
        output_file
            .write_all(
                &format!("game {} worth {} points\n", i + 1, if count > 0 {
                    (2_u32).pow(count - 1)
                } else {
                    0
                }).as_bytes()
            )
            .expect("cant write to output file");
        if count > 0 {
            total += (2_u32).pow(count - 1);
        }
    }
    let _ = output_file.write_all(format!("total: {} points", total).as_bytes());
    total
}

fn part2(input: Vec<String>, _: File) -> u32 {
    let mut total: u32 = input.len() as u32;

    let mut cards_match: HashMap<usize, u32> = HashMap::new();
    let mut memoized: HashMap<usize, u32> = HashMap::new();
    let mut cards: Vec<usize> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        let (win_nums, your_nums) = normalize_data(line.clone());
        let mut count: u32 = 0;
        for num in your_nums {
            if win_nums.contains(&num) {
                count += 1;
            }
        }
        if count > 0 {
            cards_match.entry(i).or_insert(count);
        }
        cards.push(i);
    }

    let mut idx = 0;
    while idx < cards.len() {
        let card_no = cards[idx];

        let card_val = memoize(&cards_match, &mut memoized, card_no);

        total += card_val;

        idx += 1;
    }
    println!("{}", total);
    cards.len() as u32
}

fn memoize(
    cards_match: &HashMap<usize, u32>,
    memoized: &mut HashMap<usize, u32>,
    card_no: usize
) -> u32 {
    let result: u32 = if let Some(matched) = cards_match.get(&card_no) {
        let matched_cards = Vec::from_iter(0..*matched)
            .iter()
            .map(|x| ((*x as usize) + 1 + card_no) as usize)
            .collect::<Vec<usize>>();

        let mut total: u32 = 0;
        for matched_card in matched_cards {
            total += if let Some(val) = memoized.get(&matched_card) {
                *val
            } else {
                let val = memoize(cards_match, memoized, matched_card);
                memoized.entry(matched_card).or_insert(val);
                val
            };
        }
        total + matched
    } else {
        0
    };
    memoized.entry(card_no).or_insert(result);
    result
}

fn main() {
    let input = read_input(4);

    let output_file = create_output_file(4);

    part2(input, output_file);
}
