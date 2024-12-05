use std::collections::HashMap;

use utils::read_input;

#[derive(Debug, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum FFState {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct FlipFlopModule {
    name: String,
    state: FFState,
    dest: Vec<String>,
}

impl FlipFlopModule {
    fn receive_and_send(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => { None }
            Pulse::Low => {
                let return_pulse = if let FFState::Off = self.state {
                    self.state = FFState::On;
                    Pulse::High
                } else {
                    self.state = FFState::Off;
                    Pulse::Low
                };
                Some(return_pulse)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ConjunctionModule {
    name: String,
    last_pulses: HashMap<String, Pulse>,
    dest: Vec<String>,
}

impl ConjunctionModule {
    fn update(&mut self, src: String, pulse: Pulse) {
        *self.last_pulses.get_mut(&src).unwrap() = pulse;
    }

    fn is_all_high_pulses(&self) -> bool {
        for pulse in self.last_pulses.values() {
            if let Pulse::Low = pulse {
                return false;
            }
        }

        return true;
    }

    fn receive_and_send(&mut self, src: String, pulse: Pulse) -> Pulse {
        self.update(src, pulse);

        if self.is_all_high_pulses() {
            return Pulse::Low;
        }

        return Pulse::High;
    }
}

#[derive(Debug, Clone)]
struct BroadcasterModule {
    dest: Vec<String>,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcaster(BroadcasterModule),
}

impl Module {
    fn get_name(&self) -> String {
        match self {
            Module::FlipFlop(m) => { format!("{}", m.name) }
            Module::Conjunction(m) => { format!("{}", m.name) }
            Module::Broadcaster(_) => format!("broadcaster"),
        }
    }
    fn get_name_with_prefix(&self) -> String {
        match self {
            Module::FlipFlop(m) => { format!("%{}", m.name) }
            Module::Conjunction(m) => { format!("&{}", m.name) }
            Module::Broadcaster(_) => format!("broadcaster"),
        }
    }

    fn get_destinations(&self) -> Vec<String> {
        match self {
            Module::FlipFlop(m) => m.dest.clone(),
            Module::Conjunction(m) => m.dest.clone(),
            Module::Broadcaster(m) => m.dest.clone(),
        }
    }

    fn receive_and_send(&mut self, src: String, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop(m) => { m.receive_and_send(pulse) }
            Module::Conjunction(m) => { Some(m.receive_and_send(src, pulse)) }
            Module::Broadcaster(_) => { None }
        }
    }
}

// normalize data and expand the map
fn normalize_data(input: String) -> Vec<Module> {
    input
        .split("\n")
        .map(|module| {
            let module_parts: Vec<String> = module
                .split(" -> ")
                .map(|x| x.to_string())
                .collect();
            let destinations: Vec<String> = module_parts[1]
                .split(", ")
                .map(|x| x.to_string())
                .collect();
            if module_parts[0] == "broadcaster" {
                Module::Broadcaster(BroadcasterModule { dest: destinations })
            } else {
                let module_name = module_parts[0][1..].to_string();
                if module_parts[0].starts_with("%") {
                    Module::FlipFlop(FlipFlopModule {
                        name: module_name,
                        state: FFState::Off,
                        dest: destinations,
                    })
                } else {
                    Module::Conjunction(ConjunctionModule {
                        name: module_name,
                        last_pulses: HashMap::new(),
                        dest: destinations,
                    })
                }
            }
        })
        .collect()
}

fn get_configurations(mods: Vec<Module>) -> (BroadcasterModule, HashMap<String, Module>) {
    let mut broadcaster = BroadcasterModule { dest: vec![] };
    let mut mods = mods;

    let mut mods_map: HashMap<String, Module> = HashMap::new();
    for module in &mods {
        match module {
            Module::FlipFlop(inside_mod) => {
                mods_map.entry(inside_mod.name.clone()).or_insert(module.clone());
            }
            Module::Conjunction(inside_mod) => {
                let mut connected_mods: HashMap<String, Pulse> = HashMap::new();
                for m in &mods {
                    if m.get_destinations().contains(&inside_mod.name) {
                        connected_mods.entry(m.get_name()).or_insert(Pulse::Low);
                    }
                }

                let module = Module::Conjunction(ConjunctionModule {
                    name: inside_mod.name.clone(),
                    last_pulses: connected_mods,
                    dest: inside_mod.dest.clone(),
                });
                mods_map.entry(inside_mod.name.clone()).or_insert(module.clone());
            }
            Module::Broadcaster(inside_mod) => {
                broadcaster = inside_mod.clone();
            }
        }
    }

    (broadcaster, mods_map)
}

fn part1(input: String) -> usize {
    let (broadcaster, mut modules_map) = get_configurations(normalize_data(input));
    let mut low_count: usize = 0;
    let mut high_count: usize = 0;
    for _ in 0..1000 {
        low_count += 1;
        let mut stack: Vec<(String, String, Pulse)> = Vec::new();
        for dest in broadcaster.clone().dest {
            stack.push(("broadcaster".to_string(), dest.clone(), Pulse::Low));
            low_count += 1;
        }

        while stack.len() != 0 {
            let (src, dest, pulse) = stack.remove(0);
            let pulse_string = if let Pulse::Low = pulse { "low" } else { "high" };

            // let module_ref = modules_map.get(&dest).unwrap();
            println!("{} -{} -> {}", src, pulse_string, dest);
            if modules_map.contains_key(&dest) {
                let output_pulse: Option<Pulse> = modules_map
                    .get_mut(&dest)
                    .unwrap()
                    .receive_and_send(src, pulse);
                let src: String = dest;
                if let Some(output_pulse) = output_pulse {
                    for dest in modules_map.get(&src).unwrap().get_destinations() {
                        stack.push((src.clone(), dest, output_pulse.clone()));
                        if let Pulse::Low = output_pulse {
                            low_count += 1;
                        } else {
                            high_count += 1;
                        }
                    }
                }
            }
        }
    }

    low_count * high_count
}

fn part2(input: String) -> usize {
    let (broadcaster, mut modules_map) = get_configurations(normalize_data(input));
    let mut i: usize = 0;

    loop {
        let mut stack: Vec<(String, String, Pulse)> = Vec::new();
        for dest in broadcaster.clone().dest {
            stack.push(("broadcaster".to_string(), dest.clone(), Pulse::Low));
        }

        while stack.len() != 0 {
            let (src, dest, pulse) = stack.remove(0);

            // let pulse_string = if let Pulse::Low = pulse { "low" } else { "high" };
            // println!("{} -{} -> {}", src, pulse_string, dest);

            if modules_map.contains_key(&dest) {
                let output_pulse: Option<Pulse> = modules_map
                    .get_mut(&dest)
                    .unwrap()
                    .receive_and_send(src, pulse);
                let src: String = dest;
                if let Some(output_pulse) = output_pulse {
                    for dest in modules_map.get(&src).unwrap().get_destinations() {
                        stack.push((src.clone(), dest.clone(), output_pulse.clone()));
                        if let Pulse::Low = output_pulse {
                            if dest == "rx" {
                                return i + 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        println!("i {i}");
    }

    0
}

fn main() {
    let input = read_input(20);
    // println!("{}", part1(input.clone().join("\n")));
    println!("{}", part2(input.clone().join("\n")));
}

#[cfg(test)]
mod tests {
    use crate::{ part1, part2 };

    #[test]
    fn test1() {
        assert_eq!(
            part1(r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#.to_string()),
            32000000
        );

        assert_eq!(
            part1(r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#.to_string()),
            11687500
        );
    }

    #[test]
    fn test2() {
        assert_eq!(part1(r#""#.to_string()), 0);
    }
}
