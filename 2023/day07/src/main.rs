use std::{ fs::File, collections::HashMap, cmp::Ordering, io::Write };

use utils::{ read_input, create_output_file, sort::quicksort };

fn normalize_data(input: Vec<String>) -> Vec<(String, u32)> {
    input
        .iter()
        .map(|line| {
            let parts = line
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            (parts[0].clone(), parts[1].parse::<u32>().unwrap())
        })
        .collect()
}

fn get_hand_score(hand: String) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in hand.chars() {
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
    }

    let distinct_cards = map.keys().len();
    match distinct_cards {
        5 => { 1 } // high card
        4 => { 2 } // one pair
        3 => {
            // either two pair or three of a kind
            for (_, val) in map.clone().iter() {
                if *val == 3 {
                    return 4;
                }
            }
            return 3;
        }
        2 => {
            // either full house or [four of a kind]
            for (_, val) in map.clone().iter() {
                if *val == 4 {
                    // [four of a kind]
                    return 6;
                }
            }
            // full house
            return 5;
        }
        1 => {
            // five of a kind
            7
        }
        _ => { 0 }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    pub score: usize,
    pub hand_cards: Vec<usize>,
    pub bid: u32,
    pub hand: String,
}

impl Ord for Hand {
    // only compare the hand_cards if the score is EQUAL
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.score, &self.hand_cards).cmp(&(&other.score, &other.hand_cards))
    }
}

fn part1(input: Vec<String>, mut output_file: &File) {
    let c_map: HashMap<char, usize> = HashMap::from_iter(
        vec![('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]
    );
    let mut total = 0;
    let mut hands: Vec<Hand> = Vec::new();
    for (hand, bid) in normalize_data(input) {
        let score = get_hand_score(hand.clone());
        hands.push(Hand {
            hand_cards: hand
                .chars()
                .map(|c| (
                    if c.is_numeric() {
                        c.to_string().parse::<usize>().unwrap()
                    } else {
                        *c_map.get(&c).unwrap()
                    }
                ))
                .collect(),
            bid,
            score,
            hand,
        });
    }

    quicksort(&mut hands);
    for (i, hand) in hands.iter().enumerate() {
        let _ = output_file.write_all(
            &format!(
                "hand {:?} - bid {} - score {} - rank {}\n",
                hand.hand_cards,
                hand.bid,
                hand.score,
                i + 1
            ).as_bytes()
        );
        total += ((i as u32) + 1) * hand.bid;
    }

    println!("total {total}")
}

fn get_hand_score_p2(hand: String) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in hand.chars() {
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
    }

    let distinct_cards = map.keys().len();
    let joker_count = hand
        .chars()
        .filter(|c| c == &'J')
        .count();
    match distinct_cards {
        5 => {
            // [High card] hand
            // if there is a Joker (can't be more than 1), we can make a hand of [One pair]
            if joker_count == 1 {
                // [One pair]
                2
            } else {
                // [High card]
                1
            }
        }
        4 => {
            // [One pair]
            // Joker cards count can be 0, 1, or 2
            // if it is 1 or 2, which means the pair is Jokers, we can make a [Three of a kind hand] (4 points)

            if joker_count == 2 || joker_count == 1 {
                return 4;
            }

            // the hand is still [One pair] if there is no Joker
            return 2;
        }
        3 => {
            // either [Two pair] or [Three of a kind] without taking into account the present of the Joker card(s)
            for (_, val) in map.clone().iter() {
                if *val == 3 {
                    // means that we have three of a kind
                    // if there is one or three Joker cards, we can make [Four of a kind] hand. At the point where our hand is three of a kiind, the case where there is two kind of Joker doesn't exist because there is 3 different type of cards
                    if joker_count == 3 || joker_count == 1 {
                        // if Joker count is 3, it can make the hand be [Four of a kind]
                        return 6;
                    }
                    // if there is not Joker, the hand is still [Three of a kind]
                    return 4;
                }
            }
            // at this point, we are having a hand of [Two pairs]
            // Joker card(s) can be 0, 1, or 2
            // if there is 2 Joker cards, we can make a hand of [Four of a kind] (6 points)
            if joker_count == 2 {
                return 6;
            }
            // if there is 1 Joker cards, we can make a [Full house] (5 points)
            if joker_count == 1 {
                return 5;
            }
            // if there is no Joker card, the hand is still [Two pairs]
            return 3;
        }
        2 => {
            // either [Full house] or [Four of a kind] (without taking account of Joker cards)
            for (_, val) in map.clone().iter() {
                if *val == 4 {
                    // [Four of a kind]
                    // at this point, Joker cards count can be 0, 1, 4
                    // if it is 1 or 4, we can make [Five of a kind] hand (7 points)
                    if joker_count == 1 || joker_count == 4 {
                        return 7;
                    }
                    // at this point, it's a regular hand without any Joker card
                    // still a [Four of a kind] hand
                    return 6;
                }
            }
            // [Full house]
            // Joker cards count can be 0, 2, or 3
            // if it is 2 or 3, we can make [Five of a kind] hand
            if joker_count == 2 || joker_count == 3 {
                return 7;
            }
            // still a [Full house] hand
            return 5;
        }
        1 => {
            // five of a kind
            7
        }
        _ => { 0 }
    }
}

fn part2(input: Vec<String>, mut output_file: &File) {
    // for part 2, we lower down the strength of card J
    let c_map: HashMap<char, usize> = HashMap::from_iter(
        vec![('T', 10), ('J', 1), ('Q', 12), ('K', 13), ('A', 14)]
    );

    let mut total = 0;
    let mut hands: Vec<Hand> = Vec::new();
    for (hand, bid) in normalize_data(input) {
        let score = get_hand_score_p2(hand.clone());
        hands.push(Hand {
            hand_cards: hand
                .chars()
                .map(|c| (
                    if c.is_numeric() {
                        c.to_string().parse::<usize>().unwrap()
                    } else {
                        *c_map.get(&c).unwrap()
                    }
                ))
                .collect(),
            bid,
            score,
            hand,
        });
    }

    quicksort(&mut hands);
    for (i, hand) in hands.iter().enumerate() {
        let _ = output_file.write_all(
            &format!(
                "hand {:?} - bid {} - score {} - rank {}\n",
                hand.hand,
                hand.bid,
                hand.score,
                i + 1
            ).as_bytes()
        );
        total += ((i as u32) + 1) * hand.bid;
    }

    println!("total {total}")
}

fn main() {
    let input = read_input(7);

    let output_file = create_output_file(7);

    part1(input.clone(), &output_file);
    part2(input.clone(), &output_file);
    // 10834440
}
