use std::collections::HashMap;
use itertools::Itertools;

type Num = u64;
type Workflow = Vec<Rule>;
type System = HashMap<&'static str, Workflow>;
type Part = HashMap<char, Num>;
type PartList = Vec<Part>;
type PartRange = HashMap<char, (Num, Num)>;

enum Rule {
    Rule(RuleDef),
    Final(&'static str),
}

struct RuleDef {
    category: char,
    operation: char,
    value: Num,
    destination: &'static str,
}

fn parse(input: &'static str) -> (System, PartList) {
    let mut system = System::new();
    let mut part_list = PartList::new();

    let (workflows, parts) = input.split("\n\n").map(|t| t.lines()).next_tuple().unwrap();

    for workflow in workflows {
        let mut tokens = workflow.split([',', '{', '}']).filter(|t| !t.is_empty());

        let name = tokens.next().unwrap();
        let mut workflow = Workflow::new();

        for token in tokens {
            if !token.contains(':') {
                workflow.push(Rule::Final(token));
                continue;
            }

            let mut tokens = token.split(':');

            workflow.push(Rule::Rule(RuleDef {
                category: token.chars().next().unwrap(),
                operation: token.chars().skip(1).next().unwrap(),
                value: tokens.next().unwrap()[2..].parse::<Num>().unwrap(),
                destination: tokens.next().unwrap(),
            }));
        }

        system.insert(name, workflow);
    }

    for part in parts {
        let tokens = part.split(['{', ',', '}']).filter(|t| !t.is_empty());

        let mut part = Part::new();

        for token in tokens {
            part.insert(token.chars().next().unwrap(), token[2..].parse().unwrap());
        }

        part_list.push(part);
    }


    (system, part_list)
}

fn solve_part1(system: &System, part_list: &PartList) -> Num {
    let mut rating_sum = 0;

    for part in part_list {
        let mut workflow_name = "in";

        while workflow_name != "A" && workflow_name != "R" {
            let workflow = &system[workflow_name];

            for rule in workflow {
                match rule {
                    Rule::Rule(ruledef) => {
                        let value = part[&ruledef.category];

                        let result = match ruledef.operation {
                            '<' => value < ruledef.value,
                            '>' => value > ruledef.value,
                            _   => panic!("Invalid rule operation."),
                        };

                        if result {
                            workflow_name = ruledef.destination;
                        }
                        else {
                            continue;
                        }
                    },
                    Rule::Final(new_workflow_name) => workflow_name = new_workflow_name,
                }

                break;
            }
        }

        if workflow_name == "A" {
            rating_sum += part.values().sum::<Num>();
        }
    }

    rating_sum
}

fn solve_part2(system: &System) -> Num {
    let mut parts = vec![(
        "in",
        PartRange::from([('x', (1, 4000)), ('m', (1, 4000)), ('a', (1, 4000)), ('s', (1, 4000))])
    )];
    let mut accepted_ratings = 0;

    while let Some((workflow_name, mut part_range)) = parts.pop() {
        if workflow_name == "R" {
            continue;
        }

        if workflow_name == "A" {
            let combinations = part_range.values().map(|(start, end)| end - start + 1).product::<Num>();

            accepted_ratings += combinations;
            continue;
        }

        let workflow = &system[workflow_name];

        for rule in workflow {
            match rule {
                Rule::Rule(ruledef) => {
                    let mut new_part_range = part_range.clone();

                    let value_range = part_range.entry(ruledef.category).or_default();
                    let new_value_range = new_part_range.entry(ruledef.category).or_default();

                    // Assuming there are only < and > operations.
                    if ruledef.operation == '<' {
                        value_range.0 = ruledef.value;
                        new_value_range.1 = ruledef.value - 1;
                    }
                    else {
                        value_range.1 = ruledef.value;
                        new_value_range.0 = ruledef.value + 1;
                    }

                    parts.push((ruledef.destination, new_part_range));
                },
                Rule::Final(new_workflow_name) => parts.push((new_workflow_name, part_range.clone())),
            }
        }
    }

    accepted_ratings
}

fn main() {
    let (system, part_list) = parse(include_str!("../in.txt"));

    let result_part1 = solve_part1(&system, &part_list);
    let result_part2 = solve_part2(&system);

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}