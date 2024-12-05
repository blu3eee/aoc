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

fn check_around(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut possibilities: Vec<(usize, usize)> = Vec::new();
    if x > 0 && map[x - 1][y] != '#' {
        possibilities.push((x - 1, y));
    }
    if x < map.len() - 1 && map[x + 1][y] != '#' {
        possibilities.push((x + 1, y));
    }
    if y > 0 && map[x][y - 1] != '#' {
        possibilities.push((x, y - 1));
    }
    if y < map[0].len() - 1 && map[x][y + 1] != '#' {
        possibilities.push((x, y + 1));
    }

    possibilities
}

fn print_map(map: &Vec<Vec<char>>, possible_positions: &Vec<(usize, usize)>) {
    let mut map = map.clone();
    for (x, y) in possible_positions {
        map[*x][*y] = 'O';
    }

    for row in map {
        println!(
            "{}",
            row
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
    println!()
}

fn walk(map: &Vec<Vec<char>>, steps: usize) -> usize {
    let (mut start_i, mut start_j) = (0, 0);
    // find starting point
    for (i, line) in map.iter().enumerate() {
        if line.contains(&'S') {
            start_i = i;
            start_j = line
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
                .find("S")
                .unwrap();
        }
    }

    // walk
    let mut positions: Vec<(usize, usize)> = check_around(map, start_i, start_j);

    for _ in 0..steps - 1 {
        let mut local_positions: Vec<(usize, usize)> = Vec::new();
        for (x, y) in positions.clone() {
            for (i, j) in check_around(map, x, y) {
                if !local_positions.contains(&(i, j)) {
                    local_positions.push((i, j));
                }
            }
        }
        positions = local_positions;
        print_map(map, &positions);
    }

    positions.len()
}

fn part1(input: String, steps: usize) -> usize {
    let map = normalize_data(input);
    walk(&map, steps)
}

#[allow(dead_code)]
fn part2(input: String) -> usize {
    let mut total = 0;

    total
}

fn main() {
    let input = read_input(21);
    println!("{}", part1(input.clone().join("\n"), 64));
    println!("{}", part2(input.clone().join("\n")));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(
            part1(
                r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#.to_string(),
                6
            ),
            16
        );
    }

    #[test]
    fn test2() {
        part2(r#""#.to_string());
    }
}
