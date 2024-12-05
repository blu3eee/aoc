use std::{ collections::HashMap, vec };

use utils::{ read_input, create_image::{ create_bw_image, save_image } };

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<(String, usize, String)> {
    input
        .split("\n")
        .map(|line| {
            let parts = line
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            (parts[0].clone(), parts[1].parse::<usize>().unwrap(), parts[2].clone())
        })
        .collect()
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        println!(
            "{}",
            line
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }

    println!();
}

fn draw_map(data: &Vec<(String, usize)>) -> Vec<Vec<char>> {
    let mut cur_map: Vec<Vec<char>> = vec![];

    let (mut x, mut y): (usize, usize) = (0, 0);
    for (dir, meters) in data {
        let meters = *meters;
        if cur_map.is_empty() {
            match dir.as_str() {
                "R" | "L" => {
                    cur_map = vec![vec!['#'; meters + 1]];
                    y = if dir.as_str() == "R" { meters } else { 0 };
                }
                "D" | "U" => {
                    cur_map = vec![vec!['#']; meters+1];
                    x = if dir.as_str() == "D" { meters } else { 0 };
                }
                _ => {}
            }
        } else {
            match dir.as_str() {
                "R" => {
                    let new_width = y + meters + 1;
                    if new_width > cur_map[0].len() {
                        for row in &mut cur_map {
                            row.resize(new_width, '.');
                        }
                    }
                    for i in y..y + meters + 1 {
                        cur_map[x][i] = '#';
                    }
                    y += meters;
                }
                "D" => {
                    let new_height = x + meters + 1;
                    if new_height > cur_map.len() {
                        let width = cur_map[0].len();
                        cur_map.resize(new_height, vec!['.'; width]);
                    }
                    for i in x..x + meters + 1 {
                        cur_map[i][y] = '#';
                    }

                    x += meters;
                }
                "L" => {
                    if y < meters {
                        let extend_gap = meters - y;
                        for row in &mut cur_map {
                            let mut new_row = vec!['.'; extend_gap];
                            new_row.append(row);
                            *row = new_row;
                        }
                        y = meters;
                    }
                    for i in y - meters..y {
                        cur_map[x][i] = '#';
                    }
                    y -= meters;
                }
                "U" => {
                    if x < meters {
                        let extend_gap = meters - x;
                        let width = cur_map[0].len();
                        let mut new_rows = vec![vec!['.'; width]; extend_gap];
                        new_rows.append(&mut cur_map);
                        cur_map = new_rows;
                        x = meters;
                    }
                    for i in x - meters..x {
                        cur_map[i][y] = '#';
                    }
                    x -= meters;
                }
                _ => {}
            }
        }
    }
    cur_map
}

fn count_cubes(map: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for line in map {
        total += line
            .iter()
            .filter_map(|x| if (*x as char) == '#' { Some(1) } else { None })
            .count();
    }
    total
}

fn fill_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map.clone();
    let (mut start_x, mut start_y) = (0, 0);
    for x in 1..map.len() - 1 {
        let line = &map[x];
        let mut list: Vec<usize> = Default::default();

        for (i, c) in line.iter().enumerate() {
            if (*c as char) == '#' {
                list.push(i);
            }
        }

        list = list
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if i == 0 || !list.contains(&(x - 1)) { Some(*x) } else { None })
            .collect();

        if list.len() == 2 {
            start_x = x;
            start_y = list[0] + 1;
            break;
        }
    }

    println!("before spreading");
    // print_map(&map);
    spread(&mut map, start_x, start_y);
    println!("");

    map
}

fn spread(map: &mut Vec<Vec<char>>, start_x: usize, start_y: usize) {
    if start_y != map[0].len() - 1 && map[start_x][start_y + 1] != '#' {
        map[start_x][start_y + 1] = '#';
        spread(map, start_x, start_y + 1);
    }
    if start_y != 0 && map[start_x][start_y - 1] != '#' {
        map[start_x][start_y - 1] = '#';
        spread(map, start_x, start_y - 1);
    }
    if start_x != map.len() - 1 && map[start_x + 1][start_y] != '#' {
        map[start_x + 1][start_y] = '#';
        spread(map, start_x + 1, start_y);
    }
    if start_x != 0 && map[start_x - 1][start_y] != '#' {
        map[start_x - 1][start_y] = '#';
        spread(map, start_x - 1, start_y);
    }
}

fn draw_image(map: &Vec<Vec<char>>, save_path: String) {
    let imgbuf = create_bw_image(
        &map
            .iter()
            .map(|line|
                line
                    .iter()
                    .map(|&x| if x == '#' { 0 } else { 1 })
                    .collect()
            )
            .collect()
    );

    save_image(save_path, imgbuf);
}

fn part1(input: String) -> usize {
    let data: Vec<(String, usize)> = normalize_data(input)
        .iter()
        .map(|(dir, meters, _)| (dir.to_string(), *meters))
        .collect();
    let map = draw_map(&data);
    draw_image(&map, "part1_border.png".to_string());
    let map = fill_map(&map);
    print_map(&map);
    draw_image(&map, "part1_filled.png".to_string());
    count_cubes(&map)
}

fn convert_data(data: Vec<(String, usize, String)>) -> Vec<(String, usize)> {
    data.iter()
        .map(|(_, _, color)| {
            let color = color.trim_matches('(').trim_matches(')').trim_matches('#');
            let dir = {
                let color_bit =
                    color.as_bytes().last().unwrap().to_string().parse::<usize>().unwrap() - 48;
                (
                    match color_bit {
                        0 => { "R" }
                        1 => { "D" }
                        2 => { "L" }
                        3 => { "U" }
                        _ => { "R" }
                    }
                ).to_string()
            };

            let meters = usize::from_str_radix(&color[0..color.len() - 1], 16).unwrap();
            println!("{} {}", dir, meters);
            (dir, meters)
        })
        .collect()
}

fn draw_map_2(data: &Vec<(String, usize)>) -> Vec<(usize, usize)> {
    let mut cur_map: Vec<(usize, usize)> = vec![];

    let (mut x, mut y): (usize, usize) = (0, 0);
    for (dir, meters) in data {
        let meters = *meters;
        if cur_map.is_empty() {
            match dir.as_str() {
                "R" | "L" => {
                    // cur_map = vec![vec!['#'; meters + 1]];
                    for i in 0..=meters {
                        cur_map.push((0, i));
                    }
                    y = if dir.as_str() == "R" { meters } else { 0 };
                }
                "D" | "U" => {
                    for i in 0..=meters {
                        cur_map.push((i, 0));
                    }
                    x = if dir.as_str() == "D" { meters } else { 0 };
                }
                _ => {}
            }
        } else {
            match dir.as_str() {
                "R" => {
                    for i in y..y + meters + 1 {
                        if !cur_map.contains(&(x, i)) {
                            cur_map.push((x, i));
                        }
                    }
                    y += meters;
                }
                "D" => {
                    for i in x..x + meters + 1 {
                        // cur_map[i][y] = '#';
                        if !cur_map.contains(&(i, y)) {
                            cur_map.push((i, y));
                        }
                    }

                    x += meters;
                }
                "L" => {
                    for i in y - meters..y {
                        if !cur_map.contains(&(x, i)) {
                            cur_map.push((x, i));
                        }
                    }
                    y -= meters;
                }
                "U" => {
                    for i in x - meters..x {
                        if !cur_map.contains(&(i, y)) {
                            cur_map.push((i, y));
                        }
                    }
                    x -= meters;
                }
                _ => {}
            }
        }
    }
    cur_map
}

fn spread_2(
    map: &mut HashMap<usize, Vec<usize>>,
    start_x: usize,
    start_y: usize,
    max_x: usize,
    max_y: usize
) {
    // println!("{map:?}");
    if
        start_y != max_y &&
        !map
            .get(&start_x)
            .unwrap_or(&Vec::new())
            .contains(&(start_y + 1))
    {
        map.entry(start_x)
            .or_default()
            .push(start_y + 1);
        spread_2(map, start_x, start_y + 1, max_x, max_y);
    }

    if
        start_y != 0 &&
        !map
            .get(&start_x)
            .unwrap_or(&Vec::new())
            .contains(&(start_y - 1))
    {
        map.entry(start_x)
            .or_default()
            .push(start_y - 1);
        spread_2(map, start_x, start_y - 1, max_x, max_y);
    }

    if
        start_x != max_x &&
        !map
            .get(&(start_x + 1))
            .unwrap_or(&Vec::new())
            .contains(&start_y)
    {
        map.entry(start_x + 1)
            .or_default()
            .push(start_y);
        spread_2(map, start_x + 1, start_y, max_x, max_y);
    }
    if
        start_x != 0 &&
        !map
            .get(&(start_x - 1))
            .unwrap_or(&Vec::new())
            .contains(&start_y)
    {
        map.entry(start_x - 1)
            .or_default()
            .push(start_y);
        spread_2(map, start_x - 1, start_y, max_x, max_y);
    }
}

fn fill_map_2(map: Vec<(usize, usize)>) -> HashMap<usize, Vec<usize>> {
    let mut hmap: HashMap<usize, Vec<usize>> = HashMap::new();
    for (x, y) in map {
        hmap.entry(x).or_insert(Default::default()).push(y);
    }

    let mut max_x = 0;
    let mut max_y = 0;
    for (key, y_vals) in hmap.iter() {
        max_x = max_x.max(*key);
        max_y = max_y.max(*y_vals.iter().max().unwrap());
    }
    println!("max_x {} max_y {}", max_x, max_y);
    let (mut start_x, mut start_y) = (max_x, max_y);

    for (key, y_vals) in hmap.iter() {
        if *key == 0 {
            continue;
        }
        let mut list: Vec<usize> = y_vals
            .iter()
            .enumerate()
            .filter_map(|(i, x)| (
                if i == 0 || *x == 0 || !y_vals.contains(&(x - 1)) {
                    Some(*x)
                } else {
                    None
                }
            ))

            .collect();
        list.sort();
        if list.len() == 2 {
            start_x = start_x.min(*key);
            start_y = start_y.min(list[0] + 1);
        }
    }
    println!("start_x {} start_y {}", start_x, start_y);

    spread_2(&mut hmap, start_x, start_y, max_x, max_y);

    hmap
}

fn count_cubes_2(hmap: &HashMap<usize, Vec<usize>>) -> usize {
    let mut total = 0;
    for y_vals in hmap.values() {
        total += y_vals.len();
    }
    total
}

fn part2(input: String) -> usize {
    let data: Vec<(String, usize)> = convert_data(normalize_data(input));
    println!("drawing map");
    let map: Vec<(usize, usize)> = draw_map_2(&data);
    // println!("map {:?}", map);
    println!("start filling");
    let hmap = fill_map_2(map);
    println!("map filled");
    // let data: Vec<(String, usize)> = convert_data(normalize_data(input));
    // println!("data converted {:?}", data);

    // let cur_map = draw_map(&data);
    // println!("map drawn {} x {}", cur_map[0].len(), cur_map.len());

    // draw_image(
    //     &cur_map
    //         .iter()
    //         .map(|line|
    //             line
    //                 .as_bytes()
    //                 .iter()
    //                 .map(|x| *x as char)
    //                 .collect()
    //         )
    //         .collect(),
    //     "part2_border.png".to_string()
    // );
    // let map = fill_map(&cur_map);
    // println!("map filled");

    // count_cubes(&map)
    count_cubes_2(&hmap)
    // 0
}

fn main() {
    let input = read_input(18);
    println!("{}", part1(input.clone().join("\n")));
    // println!("{}", part2(input.clone().join("\n")));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(
            part1(
                r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#.to_string()
            ),
            62
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(
                r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#.to_string()
            ),
            952408144115
        );
    }
}
