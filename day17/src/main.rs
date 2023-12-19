use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};
use itertools::Itertools;

#[derive(Clone)]
struct Node {
    came_from: usize,
    cost: u16,
    value: u8,
    dir_counter: u8,
    dir: (isize, isize),
}

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn find_path(map: &Vec<Vec<u8>>, down_limit: u8, up_limit: u8) -> u16 {
    let horizontal_range = 0..map[0].len() as isize;
    let vertical_range = 0..map.len() as isize;

    let start = (0, 0);
    let goal = (map[0].len() as isize - 1, map.len() as isize - 1);
    let mut goal_index = 0;

    let mut nodes = vec![Node { came_from: 0, cost: 0, value: map[0][0], dir_counter: 0, dir: (0, 0) }];
    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::new();

    frontier.push((Reverse(0), start, 0));

    while let Some((_, current, current_index)) = frontier.pop() {
        let current_node = nodes[current_index].clone();

        if current == goal && current_node.dir_counter >= down_limit { 
            goal_index = current_index;
            break;
        }

        for dir in DIRS {
            let neighbor = (current.0 + dir.0, current.1 + dir.1);
            let dir_counter = if current_node.dir == dir { current_node.dir_counter + 1 } else { 0 };
            let tuple = (neighbor, dir, dir_counter);

            if current_node.dir != (0, 0) && dir != current_node.dir && current_node.dir_counter + 1 < down_limit {
                continue;
            }

            if !horizontal_range.contains(&neighbor.0) || !vertical_range.contains(&neighbor.1) 
                || dir == (-current_node.dir.0, -current_node.dir.1) || dir_counter == up_limit || !visited.insert(tuple)
            {
                continue;
            }

            let cost = current_node.cost + map[neighbor.1 as usize][neighbor.0 as usize] as u16;
            nodes.push(Node {
                came_from: current_index,
                cost,
                value: map[neighbor.1 as usize][neighbor.0 as usize],
                dir_counter,
                dir,
            });
            frontier.push((Reverse(cost), neighbor, nodes.len() - 1));
        }
    }

    let mut heat_loss = 0;
    let mut node_index = goal_index;
    while node_index != 0 {
        heat_loss += nodes[node_index].value as u16;
        node_index = nodes[node_index].came_from;
    }

    heat_loss
}

fn main() {
    let map = include_str!("../in.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect_vec())
        .collect_vec();

    let result_part1 = find_path(&map, 0, 3);
    let result_part2 = find_path(&map, 4, 10);

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}