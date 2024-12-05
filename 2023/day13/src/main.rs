use utils::read_input;

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<Vec<Vec<usize>>> {
    input
        .split("\n\n")
        .map(|pattern|
            pattern
                .split("\n")
                .map(|line|
                    line
                        .as_bytes()
                        .iter()
                        .map(|c| if (*c as char) == '.' { 0 } else { 1 })
                        .collect()
                )
                .collect::<Vec<Vec<usize>>>()
        )
        .collect()
}

fn part1(input: String) -> usize {
    let mut total = 0;
    // for each pattern, check if the mirror is reflecting horizontally or vertically
    for pattern in normalize_data(input) {
        // check horizontal
        for i in 1..pattern.len() {
            let check_range = i.min(pattern.len() - i);
            let mut mirror = true;
            for gap in 0..check_range {
                if pattern[i + gap] != pattern[i - gap - 1] {
                    mirror = false;
                    break;
                }
            }
            if mirror == true {
                total += i * 100;
                break;
            }
        }

        let pattern_rotated = {
            let mut new_pattern: Vec<Vec<usize>> = Vec::new();
            for i in 0..pattern[0].len() {
                new_pattern.push(
                    pattern
                        .iter()
                        .map(|line| line[i])
                        .collect::<Vec<usize>>()
                );
            }

            new_pattern
                .iter()
                .map(|line| {
                    let mut line = line.clone();
                    line.reverse();
                    line
                })
                .collect::<Vec<Vec<usize>>>()
        };

        for i in 1..pattern_rotated.len() {
            let check_range = i.min(pattern_rotated.len() - i);
            let mut mirror = true;
            for gap in 0..check_range {
                if pattern_rotated[i + gap] != pattern_rotated[i - gap - 1] {
                    mirror = false;
                    break;
                }
            }

            if mirror == true {
                total += i;
                break;
            }
        }
    }

    total
}

fn count_diff(line1: &Vec<usize>, line2: &Vec<usize>) -> usize {
    let mut count = 0;
    for i in 0..line1.len() {
        if line1[i] != line2[i] {
            count += 1;
        }
    }

    count
}

fn part2(input: String) -> usize {
    let mut total = 0;
    // for each pattern, check if the mirror is reflecting horizontally or vertically
    for pattern in normalize_data(input) {
        let mut found_mirror = false;
        // check horizontal
        for i in 1..pattern.len() {
            let check_range = i.min(pattern.len() - i);

            let mut diff_count = 0;
            for gap in 0..check_range {
                diff_count += count_diff(&pattern[i + gap], &pattern[i - gap - 1]);
            }
            if diff_count == 1 {
                println!("horizontally at {i}  - diff: {diff_count}");
                total += i * 100;
                found_mirror = true;
                break;
            }
        }

        if found_mirror {
            continue;
        }

        let pattern_rotated = {
            let mut new_pattern: Vec<Vec<usize>> = Vec::new();
            for i in 0..pattern[0].len() {
                new_pattern.push(
                    pattern
                        .iter()
                        .map(|line| line[i])
                        .collect::<Vec<usize>>()
                );
            }

            new_pattern
                .iter()
                .map(|line| {
                    let mut line = line.clone();
                    line.reverse();
                    line
                })
                .collect::<Vec<Vec<usize>>>()
        };

        for i in 1..pattern_rotated.len() {
            let check_range = i.min(pattern_rotated.len() - i);
            let mut diff_count = 0;
            for gap in 0..check_range {
                diff_count += count_diff(&pattern_rotated[i + gap], &pattern_rotated[i - gap - 1]);
            }

            if diff_count == 1 {
                println!("vertically at {i} - diff: {diff_count}");
                total += i;
                break;
            }
        }
    }

    total
}

fn main() {
    let input = read_input(13);
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
                r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#.to_string()
            ),
            405
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(
                r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#.to_string()
            ),
            400
        );
    }
}
