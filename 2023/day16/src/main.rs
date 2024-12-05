use std::collections::{ HashMap, HashSet };

use utils::read_input;

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line|
            line
                .as_bytes()
                .iter()
                .map(|x| *x as char)
                .collect()
        )
        .collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!(
            "{}",
            line
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
    println!()
}

fn spread(
    start_i: usize,
    start_j: usize,
    direction: String,
    grid: &mut Vec<Vec<char>>,
    remember: &mut HashSet<(usize, usize, String)>
) {
    let mut cur_i: usize = start_i;
    let mut cur_j: usize = start_j;
    match direction.as_str() {
        "left" => {
            // go left
            loop {
                if grid[cur_i][cur_j] == '#' {
                } else if grid[cur_i][cur_j] == '.' {
                    grid[cur_i][cur_j] = '#';
                } else {
                    if remember.contains(&(cur_i, cur_j, direction.clone())) {
                        break;
                    }
                    remember.insert((cur_i, cur_j, direction.clone()));

                    match grid[cur_i][cur_j] {
                        '\\' => {
                            if cur_i > 0 {
                                spread(cur_i - 1, cur_j, "up".to_string(), grid, remember);
                            }
                            break;
                        }
                        '/' => {
                            if cur_i < grid.len() - 1 {
                                spread(cur_i + 1, cur_j, "down".to_string(), grid, remember);
                            }
                            break;
                        }
                        '|' => {
                            if cur_i > 0 {
                                spread(cur_i - 1, cur_j, "up".to_string(), grid, remember);
                            }
                            if cur_i < grid.len() - 1 {
                                spread(cur_i + 1, cur_j, "down".to_string(), grid, remember);
                            }
                            break;
                        }

                        _ => {}
                    }
                }
                if cur_j == 0 {
                    break;
                }
                cur_j -= 1;
            }
        }
        "right" => {
            // go left
            loop {
                if grid[cur_i][cur_j] == '#' {
                } else if grid[cur_i][cur_j] == '.' {
                    grid[cur_i][cur_j] = '#';
                } else {
                    if remember.contains(&(cur_i, cur_j, direction.clone())) {
                        break;
                    }
                    remember.insert((cur_i, cur_j, direction.clone()));
                    match grid[cur_i][cur_j] {
                        '/' => {
                            if cur_i > 0 {
                                spread(cur_i - 1, cur_j, "up".to_string(), grid, remember);
                            }
                            break;
                        }
                        '\\' => {
                            if cur_i < grid.len() - 1 {
                                spread(cur_i + 1, cur_j, "down".to_string(), grid, remember);
                            }
                            break;
                        }
                        '|' => {
                            if cur_i > 0 {
                                spread(cur_i - 1, cur_j, "up".to_string(), grid, remember);
                            }
                            if cur_i < grid.len() - 1 {
                                spread(cur_i + 1, cur_j, "down".to_string(), grid, remember);
                            }
                            break;
                        }

                        _ => {}
                    }
                }
                if cur_j == grid[0].len() - 1 {
                    break;
                }
                cur_j += 1;
            }
        }
        "down" => {
            // go left
            loop {
                if grid[cur_i][cur_j] == '#' {
                } else if grid[cur_i][cur_j] == '.' {
                    grid[cur_i][cur_j] = '#';
                } else {
                    if remember.contains(&(cur_i, cur_j, direction.clone())) {
                        break;
                    }
                    remember.insert((cur_i, cur_j, direction.clone()));
                    match grid[cur_i][cur_j] {
                        '/' => {
                            if cur_j > 0 {
                                spread(cur_i, cur_j - 1, "left".to_string(), grid, remember);
                            }
                            break;
                        }
                        '\\' => {
                            if cur_j < grid[0].len() - 1 {
                                spread(cur_i, cur_j + 1, "right".to_string(), grid, remember);
                            }
                            break;
                        }
                        '-' => {
                            if cur_j > 0 {
                                spread(cur_i, cur_j - 1, "left".to_string(), grid, remember);
                            }
                            if cur_j < grid[0].len() - 1 {
                                spread(cur_i, cur_j + 1, "right".to_string(), grid, remember);
                            }
                            break;
                        }

                        _ => {}
                    }
                }
                if cur_i == grid.len() - 1 {
                    break;
                }
                cur_i += 1;
            }
        }
        "up" => {
            // go left
            loop {
                if grid[cur_i][cur_j] == '#' {
                } else if grid[cur_i][cur_j] == '.' {
                    grid[cur_i][cur_j] = '#';
                } else {
                    if remember.contains(&(cur_i, cur_j, direction.clone())) {
                        break;
                    }
                    remember.insert((cur_i, cur_j, direction.clone()));
                    match grid[cur_i][cur_j] {
                        '\\' => {
                            if cur_j > 0 {
                                spread(cur_i, cur_j - 1, "left".to_string(), grid, remember);
                            }
                            break;
                        }
                        '/' => {
                            if cur_j < grid[0].len() - 1 {
                                spread(cur_i, cur_j + 1, "right".to_string(), grid, remember);
                            }
                            break;
                        }
                        '-' => {
                            if cur_j > 0 {
                                spread(cur_i, cur_j - 1, "left".to_string(), grid, remember);
                            }
                            if cur_j < grid[0].len() - 1 {
                                spread(cur_i, cur_j + 1, "right".to_string(), grid, remember);
                            }
                            break;
                        }

                        _ => {}
                    }
                }
                if cur_i == 0 {
                    break;
                }
                cur_i -= 1;
            }
        }
        _ => {}
    }
}

fn part1(input: String) -> usize {
    let mut remember: HashSet<(usize, usize, String)> = HashSet::new();
    let mut grid = normalize_data(input);
    // print_grid(&grid);
    spread(0, 0, "right".to_string(), &mut grid, &mut remember);
    // print_grid(&grid);
    for (i, j, _) in remember {
        grid[i][j] = '#';
    }
    // print_grid(&grid);

    grid.iter()
        .map(|line|
            line
                .iter()
                .filter_map(|&x| if x == '#' { Some(1) } else { None })
                .count()
        )
        .sum::<usize>()
}

fn part2(input: String) -> usize {
    let mut total = 0;
    let orig_grid = normalize_data(input);
    println!("left side");
    // check breams from left side
    for start_i in 0..orig_grid.len() {
        let mut remember: HashSet<(usize, usize, String)> = HashSet::new();
        let mut grid = orig_grid.clone();
        spread(start_i, 0, "right".to_string(), &mut grid, &mut remember);
        for (i, j, _) in remember {
            grid[i][j] = '#';
        }
        let tiles = grid
            .iter()
            .map(|line|
                line
                    .iter()
                    .filter_map(|&x| if x == '#' { Some(1) } else { None })
                    .count()
            )
            .sum::<usize>();

        total = total.max(tiles);
    }
    // check breams from right side
    for start_i in 0..orig_grid.len() {
        let mut remember: HashSet<(usize, usize, String)> = HashSet::new();
        let mut grid = orig_grid.clone();
        spread(start_i, grid[0].len() - 1, "left".to_string(), &mut grid, &mut remember);
        for (i, j, _) in remember {
            grid[i][j] = '#';
        }

        let tiles = grid
            .iter()
            .map(|line|
                line
                    .iter()
                    .filter_map(|&x| if x == '#' { Some(1) } else { None })
                    .count()
            )
            .sum::<usize>();

        total = total.max(tiles);
    }
    // check breams from top side
    for start_j in 0..orig_grid[0].len() {
        let mut remember: HashSet<(usize, usize, String)> = HashSet::new();
        let mut grid = orig_grid.clone();
        spread(0, start_j, "down".to_string(), &mut grid, &mut remember);
        for (i, j, _) in remember {
            grid[i][j] = '#';
        }
        total = total.max(
            grid
                .iter()
                .map(|line|
                    line
                        .iter()
                        .filter_map(|&x| if x == '#' { Some(1) } else { None })
                        .count()
                )
                .sum::<usize>()
        );
    }

    // check breams from bottom side
    for start_j in 0..orig_grid[0].len() {
        let mut remember: HashSet<(usize, usize, String)> = HashSet::new();
        let mut grid = orig_grid.clone();
        spread(grid.len() - 1, start_j, "up".to_string(), &mut grid, &mut remember);
        for (i, j, _) in remember {
            grid[i][j] = '#';
        }
        total = total.max(
            grid
                .iter()
                .map(|line|
                    line
                        .iter()
                        .filter_map(|&x| if x == '#' { Some(1) } else { None })
                        .count()
                )
                .sum::<usize>()
        );
    }

    total
}

fn main() {
    let input = read_input(16);
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
                r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#.to_string()
            ),
            46
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(
                r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#.to_string()
            ),
            51
        );
    }
}
