use std::{ fs::File, io::Write, collections::HashMap };

use utils::{ read_input, create_output_file };

fn get_src_to_dest_range_list(content: &str) -> Vec<(u64, u64, u64)> {
    content
        .split("\n")
        .skip(1)
        .map(|line| {
            let line = line
                .trim()
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (line[1], line[0], line[2])
        })
        .collect::<Vec<(u64, u64, u64)>>()
}

fn part1(input: Vec<String>, mut output_file: &File) {
    let _ = output_file.write_all(&format!("---- PART 1 ----\n").as_bytes());
    let content = input
        .join("\n")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let seeds = content[0]
        .split_at(content[0].find(":").unwrap() + 1)
        .1.trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let layers_src_to_dest = content
        .iter()
        .skip(1)
        .map(|cat| get_src_to_dest_range_list(cat))
        .collect::<Vec<Vec<(u64, u64, u64)>>>();
    let mut closest_location = u64::MAX;
    for seed in seeds {
        let _ = output_file.write_all(&format!("seed {seed}\n").as_bytes());
        let mut cur_cat_number = seed;
        for src_to_dest in &layers_src_to_dest {
            for (src, dest, range) in src_to_dest {
                if (*src..*src + *range).contains(&cur_cat_number) {
                    cur_cat_number = cur_cat_number - src + dest;
                    break;
                }
            }
        }
        closest_location = closest_location.min(cur_cat_number);
        let _ = output_file.write_all(&format!("location {cur_cat_number}\n").as_bytes());
    }

    let _ = output_file.write_all(&format!("closest location {closest_location}\n").as_bytes());
}

fn part2(input: Vec<String>, mut output_file: &File) {
    let _ = output_file.write_all(&format!("---- PART 2 ----\n").as_bytes());
    let content = input
        .join("\n")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let seeds = content[0]
        .split_at(content[0].find(":").unwrap() + 1)
        .1.trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();
    for i in 0..seeds.len() / 2 {
        seed_ranges.push((seeds[i * 2], seeds[i * 2 + 1]));
    }

    let layers_src_to_dest = content
        .iter()
        .skip(1)
        .map(|cat| get_src_to_dest_range_list(cat))
        .collect::<Vec<Vec<(u64, u64, u64)>>>();

    let mut closest_location = u64::MAX;
    let mut memoizeds: Vec<
        HashMap<(u64, u64), u64>
    > = vec![HashMap::new(); layers_src_to_dest.len()];
    for (init_seed, range) in seed_ranges {
        println!("seed range {init_seed} , {range}");
        let _ = output_file.write_all(&format!("seed range {init_seed} , {range}\n").as_bytes());
        let mut seed = init_seed;
        while seed < init_seed + range {
            // println!("seed {seed}");
            let mut cur_layer_src = seed;

            let mut path: Vec<u64> = Vec::new();
            let mut step: u64 = u64::MAX;
            for (i, layer_vec) in layers_src_to_dest.clone().iter().enumerate() {
                // println!("layer {i}");
                let mut found = false;
                for ((start, end), value) in memoizeds[i].iter() {
                    if (*start..*end).contains(&cur_layer_src) {
                        // println!("found memoized {} from {:?}", cur_layer_src, memoizeds[i]);
                        cur_layer_src = *value;
                        found = true;
                        step = end - start;
                        break;
                    }
                }
                if found {
                    break;
                }

                for (src, dest, range) in layer_vec.clone() {
                    if (src..src + range).contains(&cur_layer_src) {
                        println!("{:?} {}", (src, dest, range), cur_layer_src);
                        path.push(cur_layer_src);
                        step = step.min(src + range - cur_layer_src);
                        cur_layer_src = cur_layer_src - src + dest;
                        println!("not found");
                        break;
                    }
                }
            }

            if step == u64::MAX {
                println!("not found");
                step = 1;
            }
            closest_location = closest_location.min(cur_layer_src);
            let _ = output_file.write_all(&format!("location: {cur_layer_src}\n").as_bytes());
            for (i, start) in path.iter().enumerate() {
                memoizeds[i].entry((*start, *start + step)).or_insert(closest_location);
            }

            let _ = output_file.write_all(&format!("seed step: {step}\n").as_bytes());

            seed += step;
        }
    }

    let _ = output_file.write_all(&format!("closest location {closest_location}\n").as_bytes());
}

fn main() {
    let input = read_input(5);

    let output_file = create_output_file(5);

    part1(input.clone(), &output_file);
    part2(input.clone(), &output_file);
    // 10834440
}
