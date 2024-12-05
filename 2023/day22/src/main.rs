use std::collections::HashMap;

use utils::read_input;

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<Vec<usize>> {
    todo!()
}

fn part1(input: String) -> usize {
    let mut total = 0;

    total
}

fn part2(input: String) -> usize {
    let mut total = 0;

    total
}

fn main() {
    let input = read_input(22);
    println!("{}", part1(input.clone().join("\n")));
    // println!("{}", part2(input.clone().join("\n")));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(part1(r#""#.to_string()), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(part1(r#""#.to_string()), 0);
    }
}
