use std::collections::HashSet;
use polynomial::Polynomial;

fn simulate(steps: usize, garden: &Vec<Vec<char>>, border_enabled: bool) -> usize {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    let vertial_range = 0..garden.len() as isize;
    let horizontal_range = 0..garden[0].len() as isize;

    let start = garden.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter()
            .enumerate()
            .filter(|&(_, &c)| c == 'S')
            .next()
            .map(|(x, _)| (x as isize,y as isize))
        ).unwrap();
    let mut frontier = HashSet::from([start]);

    for _ in 0..steps {
        let current_layer = frontier.drain().collect::<Vec<_>>();

        for pos in current_layer.iter() {
            for dir in DIRS {
                let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);

                if border_enabled {
                    if !horizontal_range.contains(&neighbor_pos.0) || !vertial_range.contains(&neighbor_pos.1) {
                        continue;
                    }
                }

                let x = ((neighbor_pos.0 % horizontal_range.end) + horizontal_range.end) % horizontal_range.end;
                let y = ((neighbor_pos.1 % vertial_range.end) + vertial_range.end) % vertial_range.end;
                
                if garden[y as usize][x as usize] == '#' {
                    continue;
                }

                frontier.insert(neighbor_pos);
            }
        }
    }

    frontier.len()
}

fn solve_part2(garden: &Vec<Vec<char>>) -> usize {
    const STEPS: isize = 26501365;
    const SQUARE_SIZE: usize = 65;

    let values = (0..3).map(|i| simulate(SQUARE_SIZE + i * garden.len(), &garden, false) as isize + 1)
        .collect::<Vec<_>>();
    let poly = Polynomial::lagrange(&[0, 1, 2], &values).unwrap();

    poly.eval(STEPS / garden.len() as isize) as usize - 1
}

fn main() {
    let garden = include_str!("../in.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result_part1 = simulate(64, &garden, true);
    let result_part2 = solve_part2(&garden);
    
    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}