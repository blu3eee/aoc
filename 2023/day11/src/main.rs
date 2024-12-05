use std::{ fs::File, collections::HashMap, cmp::Ordering, io::Write };

use utils::{ read_input, create_output_file, sort::quicksort };

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<Vec<usize>> {
    let lines = input
        .split("\n")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let mut empty_rows = Vec::from_iter(0..lines.len());
    let mut empty_cols = Vec::from_iter(0..lines[0].len());
    let mut map: Vec<Vec<usize>> = vec![vec![0; lines[0].len()];lines.len()];
    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if (lines[i].as_bytes()[j] as char) != '.' {
                count += 1;
                map[i][j] = count;
                // println!("map {:?}", map);
                if let Some(idx) = empty_cols.iter().position(|&x| x == j) {
                    empty_cols.remove(idx);
                }
                if let Some(idx) = empty_rows.iter().position(|&x| x == i) {
                    empty_rows.remove(idx);
                }
            }
        }
    }
    empty_cols.reverse();
    for col_idx in empty_cols {
        for i in 0..map.len() {
            map[i].insert(col_idx, 0);
        }
    }
    empty_rows.reverse();
    for row_idx in empty_rows {
        map.insert(row_idx, vec![0; map[0].len()]);
    }
    map
}

fn sum_short_paths(map: Vec<Vec<usize>>) -> u64 {
    let mut results: Vec<(usize, usize)> = Vec::new();
    let mut total: u64 = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            if point != &0 {
                if results.len() > 0 {
                    for (x, y) in results.clone() {
                        total += (x.abs_diff(i) as u64) + (y.abs_diff(j) as u64);
                    }
                }
                results.push((i, j));
            }
        }
    }

    total
}

fn part1(input: String) -> u64 {
    let map = normalize_data(input);
    sum_short_paths(map)
}

fn normalize_data_part2(input: String) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
    let lines = input
        .split("\n")
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let mut empty_rows: Vec<usize> = Vec::from_iter(0..lines.len());
    let mut empty_cols: Vec<usize> = Vec::from_iter(0..lines[0].len());
    let mut map: Vec<Vec<usize>> = vec![vec![0; lines[0].len()];lines.len()];
    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if (lines[i].as_bytes()[j] as char) != '.' {
                count += 1;
                map[i][j] = count;
                // println!("map {:?}", map);
                if let Some(idx) = empty_cols.iter().position(|&x| x == j) {
                    empty_cols.remove(idx);
                }
                if let Some(idx) = empty_rows.iter().position(|&x| x == i) {
                    empty_rows.remove(idx);
                }
            }
        }
    }
    (map, empty_rows, empty_cols)
}

fn find_shortest_paths_part2(
    map: Vec<Vec<usize>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    distance_multiplier: u64
) -> u64 {
    let mut results: Vec<(usize, usize)> = Vec::new();
    let mut total: u64 = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            if point != &0 {
                if results.len() > 0 {
                    for (x, y) in results.clone() {
                        total += (x.abs_diff(i) as u64) + (y.abs_diff(j) as u64);
                        // check for empty rows or columns
                        if y != j {
                            let elapsed_cols = empty_cols
                                .iter()
                                .filter_map(|col| (
                                    if Vec::from_iter(j.min(y)..j.max(y)).contains(col) {
                                        Some(1)
                                    } else {
                                        None
                                    }
                                ))
                                .count();
                            total += (elapsed_cols as u64) * (distance_multiplier - 1);
                        }
                        if x != i {
                            let elapsed_rows = empty_rows
                                .iter()
                                .filter_map(|row| (
                                    if Vec::from_iter(i.min(x)..i.max(x)).contains(row) {
                                        Some(1)
                                    } else {
                                        None
                                    }
                                ))
                                .count();
                            total += (elapsed_rows as u64) * (distance_multiplier - 1);
                        }
                    }
                }
                results.push((i, j));
            }
        }
    }

    total
}

fn part2(input: String, distance_multiplier: u64) -> u64 {
    let (map, empty_rows, empty_cols) = normalize_data_part2(input);

    find_shortest_paths_part2(map, empty_rows, empty_cols, distance_multiplier)
}

fn main() {
    let input = read_input(11);
    println!("{}", part1(input.clone().join("\n")));
    println!("{}", part2(input.clone().join("\n"), 1000000));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(
            part1(
                r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#.to_string()
            ),
            374
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(
                r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#.to_string(),
                10
            ),
            1030
        );

        assert_eq!(
            part2(
                r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#.to_string(),
                100
            ),
            8410
        );
    }
}
