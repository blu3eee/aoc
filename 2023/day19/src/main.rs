use std::collections::HashMap;

use utils::read_input;

#[derive(Debug, Clone)]
struct MachinePart {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl MachinePart {
    fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<String>,
}

// normalize data and expand the map
fn normalize_data(input: String) -> (HashMap<String, Workflow>, Vec<MachinePart>) {
    let parts: Vec<String> = input
        .split("\n\n")
        .map(|part| part.to_string())
        .collect();

    let workflows: Vec<Workflow> = parts[0]
        .clone()
        .split("\n")
        .map(|wf| {
            let workflow_parts: Vec<String> = wf
                .split("{")
                .map(|x| x.trim().to_string())
                .collect();
            let name = workflow_parts[0].clone();
            let mut rules = workflow_parts[1].clone();
            rules.pop();
            let rules: Vec<String> = rules
                .split(",")
                .map(|x| x.to_string())
                .collect();

            Workflow {
                name,
                rules,
            }
        })
        .collect();
    let machine_parts: Vec<MachinePart> = parts[1]
        .clone()
        .split("\n")
        .map(|machine_part| {
            let cats: Vec<u32> = machine_part[1..machine_part.len() - 1]
                .split(",")
                .map(|cat| {
                    cat.split("=")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()[1]
                        .parse::<u32>()
                        .unwrap()
                })
                .collect();
            MachinePart {
                x: cats[0],
                m: cats[1],
                a: cats[2],
                s: cats[3],
            }
        })
        .collect();

    let mut workflows_hashmap: HashMap<String, Workflow> = HashMap::new();
    for wf in &workflows {
        workflows_hashmap.entry(wf.name.clone()).or_insert(wf.clone());
    }
    (workflows_hashmap, machine_parts)
}

fn rule_compare(rule: &str, value: u32) -> bool {
    match rule.as_bytes()[1] as char {
        '>' => {
            return value > rule[2..].parse::<u32>().unwrap();
        }
        '<' => {
            return value < rule[2..].parse::<u32>().unwrap();
        }
        _ => {
            return false;
        }
    }
}

fn check_rule(machine_part: &MachinePart, rule: &str) -> bool {
    match rule.as_bytes()[0] as char {
        'x' => {
            return rule_compare(rule, machine_part.x);
        }
        'm' => {
            return rule_compare(rule, machine_part.m);
        }
        'a' => {
            return rule_compare(rule, machine_part.a);
        }
        's' => {
            return rule_compare(rule, machine_part.s);
        }
        _ => {
            return false;
        }
    }
}

fn check_acceptance(
    workflows_hashmap: &HashMap<String, Workflow>,
    machine_part: &MachinePart
) -> bool {
    let mut current_wf_name: String = "in".to_string();
    let mut state = false;
    loop {
        let current_wf = workflows_hashmap.get(&current_wf_name).unwrap();
        println!("current wf: {}", current_wf_name);

        for rule in &current_wf.rules {
            // println!("rule {rule}");
            current_wf_name = rule.to_string();
            if rule.len() == 1 {
                break;
            }
            let rule_parts = &rule
                .split(":")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if check_rule(machine_part, &rule_parts[0]) {
                current_wf_name = rule_parts[1].clone();
                break;
            }
        }

        state = current_wf_name == "A";
        if current_wf_name == "A" || current_wf_name == "R" {
            println!("break");
            break;
        }
    }

    state
}

fn part1(input: String) -> u32 {
    let (workflows_hashmap, machine_parts) = normalize_data(input);
    println!("{:?}\n\n{:?}", workflows_hashmap, machine_parts);
    let mut total = 0;
    for machine_part in machine_parts {
        println!("machine part: {:?}", machine_part);
        if check_acceptance(&workflows_hashmap, &machine_part) {
            println!("accepted");
            total += machine_part.value();
        }
        println!();
    }

    total
}

fn part2(input: String) -> usize {
    let mut total = 0;

    total
}

fn main() {
    let input = read_input(19);
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
                r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#.to_string()
            ),
            19114
        );
    }

    #[test]
    fn test2() {
        assert_eq!(part1(r#""#.to_string()), 0);
    }
}
