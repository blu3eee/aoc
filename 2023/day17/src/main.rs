use utils::read_input;

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<Vec<usize>> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            println!("{line}");
            line.trim()
                .split("")
                .filter_map(|x| if x != "" { Some(x) } else { None })
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
#[derive(PartialEq, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
    map: Vec<Vec<usize>>,
}

impl Point {
    fn go_left(&mut self) {
        self.y = self.y - 1;
    }

    fn go_right(&mut self) {
        self.y = self.y + 1;
    }

    fn go_up(&mut self) {
        self.x = self.x - 1;
    }

    fn go_down(&mut self) {
        self.x = self.x + 1;
    }

    fn can_go_left(&self) -> bool {
        self.y != 0
    }

    fn can_go_right(&self) -> bool {
        self.y != self.map[0].len() - 1
    }

    fn can_go_up(&self) -> bool {
        self.x != 0 && self.y != self.map[0].len() - 1
    }

    fn can_go_down(&self) -> bool {
        self.x != self.map.len() - 1
    }
}

fn go_path(
    map: &Vec<Vec<usize>>,
    start_point: Point,
    direction: &str,
    heat_loss: usize,
    prev_path: Vec<Point>,
    min_heat_loss: usize
    // mut : &File
) -> usize {
    let mut cur_count: usize = 0;
    let mut cur_heat_loss = heat_loss;
    let mut cur_point = start_point;
    let mut cur_path = prev_path;
    let mut cur_direction = direction;
    let mut min_heat_loss = min_heat_loss;
    loop {
        cur_heat_loss += map[cur_point.x][cur_point.y];
        // let _ = .write_all(
        //     &format!(
        //         "[{}] ({}, {}) - cur_heat_loss {}\n",
        //         cur_direction,
        //         cur_point.x,
        //         cur_point.y,
        //         cur_heat_loss
        //     ).as_bytes()
        // );
        // println!();
        if cur_path.contains(&cur_point) {
            return min_heat_loss;
        }
        cur_path.push(cur_point.clone());

        if cur_heat_loss > min_heat_loss {
            return min_heat_loss;
        }

        if cur_point.x == map.len() - 1 && cur_point.y == map[0].len() - 1 {
            break;
        }

        match cur_direction {
            "right" => {
                if cur_count < 2 {
                    
                    if cur_point.can_go_down() {
                        let mut down_point = cur_point.clone();
                        down_point.go_down();
                        let go_down_heat = go_path(
                            map,
                            down_point,
                            "down",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );
                        min_heat_loss = min_heat_loss.min(go_down_heat);
                    }
                    if cur_point.can_go_up() {
                        let mut up_point = cur_point.clone();
                        up_point.go_up();
                        let go_up_heat = go_path(
                            map,
                            up_point,
                            "up",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );
                        min_heat_loss = min_heat_loss.min(go_up_heat);
                    }
                }

                if cur_point.y == map[0].len() - 1 || cur_count == 2 {
                    cur_count = 0;
                    if !cur_point.can_go_up() {
                        cur_point.go_down();
                        cur_direction = "down";
                    } else if !cur_point.can_go_down() {
                        cur_point.go_up();
                        cur_direction = "up";
                    } else {
                        let mut up_point = cur_point.clone();
                        up_point.go_up();
                        let go_up_heat = go_path(
                            map,
                            up_point,
                            "up",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );
                        min_heat_loss = min_heat_loss.min(go_up_heat);

                        let mut down_point = cur_point.clone();
                        down_point.go_down();
                        let go_down_heat = go_path(
                            map,
                            down_point,
                            "down",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );

                        return min_heat_loss.min(go_down_heat);
                    }
                } else {
                    cur_count += 1;
                    cur_point.go_right();
                }
            }
            "down" => {
                if cur_count < 2 {
                    if cur_point.can_go_right() {
                        let mut right_point = cur_point.clone();
                        right_point.go_right();
                        let go_right_heat = go_path(
                            map,
                            right_point,
                            "right",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );
                        min_heat_loss = min_heat_loss.min(go_right_heat);
                    }
                    if cur_point.can_go_left() {
                        let mut left_point = cur_point.clone();
                        left_point.go_left();
                        let go_right_heat = go_path(
                            map,
                            left_point,
                            "left",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );
                        min_heat_loss = min_heat_loss.min(go_right_heat);
                    }
                }

                if cur_point.x == map.len() - 1 {
                    cur_count = 0;
                    cur_point.go_right();
                    cur_direction = "right";
                } else if cur_count == 2 {
                    if !cur_point.can_go_left() {
                        let mut right_point = cur_point.clone();
                        right_point.go_right();
                        let go_down_heat = go_path(
                            map,
                            right_point,
                            "right",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );

                        return min_heat_loss.min(go_down_heat);
                    } else if !cur_point.can_go_right() {
                        let mut left_point = cur_point.clone();
                        left_point.go_left();
                        let go_left_heat = go_path(
                            map,
                            left_point,
                            "left",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );
                        return min_heat_loss.min(go_left_heat);
                    } else {
                        let mut left_point = cur_point.clone();
                        left_point.go_left();
                        let go_left_heat = go_path(
                            map,
                            left_point,
                            "left",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );
                        let min_heat_loss = min_heat_loss.min(go_left_heat);

                        let mut right_point = cur_point.clone();
                        right_point.go_right();
                        let go_down_heat = go_path(
                            map,
                            right_point,
                            "right",
                            cur_heat_loss,
                            cur_path.clone(),
                            min_heat_loss
                        );

                        return min_heat_loss.min(go_down_heat);
                    }
                } else {
                    cur_count += 1;
                    cur_point.go_down();
                }
            }
            "left" => {
                if cur_count < 2 && cur_point.can_go_down() {
                    let mut down_point = cur_point.clone();
                    down_point.go_down();
                    let go_down_heat = go_path(
                        map,
                        down_point,
                        "down",
                        cur_heat_loss,
                        cur_path.clone(),
                        min_heat_loss
                    );
                    min_heat_loss = min_heat_loss.min(go_down_heat);
                }

                if cur_point.y == 0 || cur_count == 2 {
                    cur_count = 0;
                    if cur_point.x == map.len() - 1 {
                        return min_heat_loss;
                    }
                    cur_point.go_down();
                    cur_direction = "down";
                } else {
                    cur_count += 1;
                    cur_point.go_left();
                }
            }

            "up" => {
                if cur_count < 2 && cur_point.can_go_right() {
                    let mut right_point = cur_point.clone();
                    right_point.go_right();
                    let go_right_heat = go_path(
                        map,
                        right_point,
                        "right",
                        cur_heat_loss,
                        cur_path.clone(),
                        min_heat_loss
                    );
                    min_heat_loss = min_heat_loss.min(go_right_heat);
                }

                if cur_point.x == 0 || cur_count == 2 {
                    cur_count = 0;
                    if cur_point.y == map[0].len() - 1 {
                        return min_heat_loss;
                    }
                    cur_point.go_right();
                    cur_direction = "right";
                } else {
                    cur_count += 1;
                    cur_point.go_up();
                }
            }
            _ => {}
        }
        if cur_heat_loss > min_heat_loss {
            return min_heat_loss;
        }
    }

    print_path(map, cur_path, cur_heat_loss);
    println!("heat loss: {cur_heat_loss}\n");
    cur_heat_loss
}

fn print_path(map: &Vec<Vec<usize>>, prev_path: Vec<Point>, heat_loss: usize) {
    let mut printing_map = map
        .iter()
        .map(|line|
            line
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
        .collect::<Vec<String>>();
    for point in prev_path {
        printing_map[point.x].replace_range(point.y..point.y + 1, ".");
    }
    for line in printing_map {
        println!("{line}");
    }

    println!("heat loss: {heat_loss}\n");
}

fn part1(input: String) -> usize {
    let city_map = normalize_data(input);
    let mut start_point = Point { x: 0, y: 0, map: city_map.clone() };
    let mut go_right = start_point.clone();
    go_right.go_right();
    // let  = create_(17);
    let result = go_path(
        &city_map,
        go_right,
        "right",
        0,
        Default::default(),
        usize::MAX
        // &
    );
    println!("min heat loss go right: {}", result);
    let mut total = 0;
    total
}

fn part2(input: String) -> usize {
    let mut total = 0;

    total
}

fn main() {
    let input = read_input(17);
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
                r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#.to_string()
            ),
            0
        );
    }

    #[test]
    fn test2() {
        // assert_eq!(part1(r#""#.to_string()), 0);
    }
}
