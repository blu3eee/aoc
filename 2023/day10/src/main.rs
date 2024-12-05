use std::{ fs::File, collections::HashMap, cmp::Ordering, io::Write };

use utils::{ read_input, create_output_file, sort::quicksort };

fn normalize_data(input: String) -> ((usize, usize), Vec<Vec<Tile>>) {
    let (mut start_i, mut start_j): (usize, usize) = (0, 0);
    let tiles = input
        .split("\n")
        .enumerate()
        .map(|(i, line)|
            line
                .split("")
                .enumerate()
                .map(|(j, x)| {
                    let tile = Tile::from(x.to_string());
                    if let Tile::StartingPoint = tile {
                        start_i = i;
                        start_j = j;
                    }

                    tile
                })
                .collect::<Vec<Tile>>()
        )
        .collect::<Vec<Vec<Tile>>>();

    ((start_i, start_j), tiles)
}

enum Tile {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    StartingPoint,
}

impl From<String> for Tile {
    fn from(value: String) -> Self {
        match value.as_str() {
            "|" => { Tile::Vertical }
            "-" => { Tile::Horizontal }
            "L" => { Tile::NorthToEast }
            "J" => { Tile::NorthToWest }
            "7" => { Tile::SouthToWest }
            "F" => { Tile::SouthToEast }
            "S" => { Tile::StartingPoint }
            _ => { Tile::Ground }
        }
    }
}

fn part1(input: String) -> u32 {
    let ((i, j), tiles) = normalize_data(input);
    let directions = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];

    for (i_diff, j_diff) in directions {
        let cur_i: isize = (i as isize) + i_diff;
        let cur_j: isize = (j as isize) + j_diff;
        let mut found_loop = false;

        while
            cur_i >= 0 &&
            cur_i < (tiles.len() as isize) &&
            cur_j >= 0 &&
            cur_j < (tiles[0].len() as isize)
        {
            let cur_tile = &tiles[cur_i as usize][cur_j as usize];
        }
    }
    0
}

enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

// check the loop if you go that direction
// fn check_loop(tiles: Vec<Vec<Tile>>, current_point: Tile, direction: Direction) -> bool {
//     match current_point {
//         Tile::Vertical => {

//         }
//         Tile::Horizontal => todo!(),
//         Tile::NorthToEast => todo!(),
//         Tile::NorthToWest => todo!(),
//         Tile::SouthToWest => todo!(),
//         Tile::SouthToEast => todo!(),
//         Tile::Ground => todo!(),
//         Tile::StartingPoint => todo!(),
//     }
//     // Some(current_point)
// }

fn part2(input: String) {}

fn main() {
    let input = read_input(10);
    part1(input.clone().join("\n"));
    part2(input.clone().join("\n"));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test1() {
        assert_eq!(part1(r#".....
.S-7.
.|.|.
.L-J.
.....
        "#.to_string()), 4);
        assert_eq!(part1(r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
        "#.to_string()), 8);
    }
}
