use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;
use num::integer;

fn main() {
    let re = Regex::new("[A-Z]+").unwrap();
    let mut input = re.find_iter(include_str!("../in.txt"))
        .map(|m| m.as_str());
    let path = input.next().unwrap().chars().collect::<Vec<_>>();
    let mut currents = Vec::new();
    let nodes = input.tuples().fold(HashMap::new(), |mut map, (start, left, right)| {
        if start.chars().last().unwrap() == 'A' {
            currents.push(start);
        }

        map.insert(start, (left, right));
        map
    });

    let mut current = "AAA";
    let mut steps_part1 = 0;
    loop {
        current = if path[steps_part1 % path.len()] == 'L' { nodes[&current].0 } else { nodes[&current].1 };
        steps_part1 += 1;

        if current == "ZZZ" {
            break;
        }
    }

    let mut steps = vec![0_u64; currents.len()];
    for i in 0..steps.len() {
        loop {
            currents[i] = if path[(steps[i] % path.len() as u64) as usize] == 'L' {
                nodes[&currents[i]].0
            }
            else {
                nodes[&currents[i]].1
            };
            steps[i] += 1;
    
            if currents[i].chars().last().unwrap() == 'Z' {
                break;
            }
        }
    }
    let steps_part2 = steps.iter().fold(1, |current, n| integer::lcm(current, *n as u64));

    println!("part1: {}", steps_part1);
    println!("part2: {}", steps_part2);
}
