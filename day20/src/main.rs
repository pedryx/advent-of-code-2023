use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
struct Module {
    state: bool,
    memmory: HashMap<&'static str, bool>,
    outputs: Vec<&'static str>,
    module_type: ModuleType,
}

#[derive(PartialEq, Eq, Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

impl Module {
    fn process_pulse(&mut self, value: bool, input: &'static str) -> Option<bool> {
        match self.module_type {
            ModuleType::FlipFlop => {
                if value {
                    None
                }
                else {
                    self.state = !self.state;
                    Some(self.state)
                }
            }
            ModuleType::Conjunction => {
                *self.memmory.get_mut(input).unwrap() = value;

                Some(!self.memmory.values().all(|&v| v))
            }
            ModuleType::Broadcast => Some(false),
        }
    }
}

fn simulate(module_map: &mut HashMap<&str, Module>) ->  (u64, u64) {
    let mut queue = VecDeque::from([("roadcaster", "button", false)]);

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    while let Some((module_name, in_name, in_value)) = queue.pop_front() {
        if in_value {
            high_pulse_count += 1;
        }
        else {
            low_pulse_count += 1;
        }

        if !module_map.contains_key(module_name) { continue; }

        let module = module_map.get_mut(module_name).unwrap();

        if let Some(out_value) = module.process_pulse(in_value, in_name) {
            queue.extend(module.outputs.iter().map(|&name| (name, module_name, out_value)));
        }
    }

    (low_pulse_count, high_pulse_count)
}

fn solve_part2(module_map: &HashMap<&str, Module>) -> u64 {
    module_map["roadcaster"].outputs.iter().map(|module_name| {
        let mut number = 0;
        let mut pow = 1;
        let mut module = &module_map[module_name];

        while !(module.outputs.len() == 1 && module_map[module.outputs[0]].module_type == ModuleType::Conjunction) {
            for output in module.outputs.iter() {
                let output_module = &module_map[output];

                match output_module.module_type {
                    ModuleType::FlipFlop => module = &output_module,
                    ModuleType::Conjunction => number += pow,
                    _ => (),
                };
            }

            pow <<= 1;
        }

        number + pow
    }).product()
}

fn parse(input: &'static str) -> HashMap<&'static str, Module> {
    let mut conjunctions = Vec::new();
    let mut module_map = input.lines().map(|l| {
        let (module, outputs): (&'static str, &'static str) = l.split_once(" -> ").unwrap();

        (&module[1..], Module {
            state: false,
            memmory: HashMap::new(),
            outputs: outputs.split(", ").collect(),
            module_type: match module.chars().next().unwrap() {
                'b' => ModuleType::Broadcast,
                '%' => ModuleType::FlipFlop,
                '&' => { conjunctions.push(&module[1..]); ModuleType::Conjunction },
                _   => panic!("Invalid module type.")
            }
        })
    }).collect::<HashMap<_, _>>();

    let keys = module_map.keys().map(|&k| k).collect::<Vec<_>>();

    for conjunction_key in conjunctions {
        for &key in keys.iter() {
            if module_map[key].outputs.contains(&conjunction_key) {
                module_map.get_mut(conjunction_key).unwrap().memmory.insert(key, false);
            }
        }

    }

    module_map
}

fn main() {
    let mut module_map_part1 = parse(include_str!("../in.txt"));
    let module_map_part2 = module_map_part1.clone();

    let result_part1 = (0..1000)
        .map(|_| simulate(&mut module_map_part1))
        .fold((0, 0), |acc, (low, high)| (acc.0 + low, acc.1 + high));
    let result_part2 = solve_part2(&module_map_part2);

    println!("part1: {}", result_part1.0 * result_part1.1);
    println!("part2: {}", result_part2);
}