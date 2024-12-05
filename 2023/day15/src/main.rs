use std::{ collections::HashMap, ops::Index };

use utils::read_input;

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<String> {
    input
        .split(",")
        .map(|x| x.to_string())
        .collect()
}

fn calculate_hash(input: String) -> usize {
    let mut val: usize = 0;
    for c in input.as_bytes() {
        val += *c as usize;
        val *= 17;
        val %= 256;
    }
    // println!("input {} - val {}", input, val);

    val
}

fn part1(input: String) -> usize {
    let mut total = 0;
    for input in normalize_data(input) {
        total += calculate_hash(input);
    }
    total
}

#[derive(Debug)]
struct Box {
    label: String,
    focal: usize,
}

fn part2(input: String) -> usize {
    let mut boxes: HashMap<usize, Vec<Box>> = HashMap::new();
    for lens in normalize_data(input) {
        if lens.contains("=") {
            let parts = lens
                .split("=")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let label_hash = calculate_hash(parts[0].clone());
            let hashed_boxes = boxes.entry(label_hash).or_insert(Default::default());
            if
                hashed_boxes
                    .iter()
                    .filter_map(|x| if x.label == parts[0] { Some(1) } else { None })
                    .count() == 1
            {
                for hashed_box in hashed_boxes {
                    if hashed_box.label == parts[0] {
                        hashed_box.focal = parts[1].parse::<usize>().unwrap();
                        break;
                    }
                }
            } else {
                hashed_boxes.push(Box {
                    label: parts[0].clone(),
                    focal: parts[1].parse::<usize>().unwrap(),
                });
            }
        } else if lens.contains("-") {
            let parts = lens
                .split("-")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let label_hash = calculate_hash(parts[0].clone());
            let hashed_boxes = boxes.entry(label_hash).or_insert(Default::default());
            if
                hashed_boxes
                    .iter()
                    .filter_map(|x| if x.label == parts[0] { Some(1) } else { None })
                    .count() == 1
            {
                hashed_boxes.remove(
                    hashed_boxes
                        .iter()
                        .position(|x| x.label == parts[0])
                        .unwrap()
                );
                if hashed_boxes.len() == 0 {
                    boxes.remove(&label_hash);
                }
            }
        }
    }
    let mut total = 0;
    for (key, box_vec) in boxes.iter() {
        println!("box_vec {box_vec:?}");
        for (i, boxed_len) in box_vec.iter().enumerate() {
            println!("key {} slot {} focal {}", key + 1, i + 1, boxed_len.focal);
            total += (key + 1) * (i + 1) * boxed_len.focal;
        }
    }
    total
}

fn main() {
    let input = read_input(15);
    println!("{}", part1(input.clone().join("\n")));
    println!("{}", part2(input.clone().join("\n")));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(
            part1(r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#.to_string()),
            1320
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#.to_string()),
            145
        );
    }
}
