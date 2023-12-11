use itertools::Itertools;

type Num = i64;

fn parse_input(input: &str) -> Vec<[Num; 2]> {
    input.lines()
        .enumerate()
        .map(|(y, l)| l.chars().enumerate().filter(|&(_, c)| c == '#').map(move |(x, _)| [x as Num, y as Num]))
        .flatten()
        .collect_vec()
}

fn solve(mut galaxies: Vec<[Num; 2]>, offset: Num) -> Num {
    let width = galaxies.iter().max_by_key(|galaxy| galaxy[0]).unwrap()[0];
    let height = galaxies.iter().max_by_key(|galaxy| galaxy[1]).unwrap()[1];
    let mut size = [width, height];

    for i in 0..2 {
        let mut pos = 0;
        while pos <= size[i] {
            if galaxies.iter().all(|&galaxy| galaxy[i] != pos) {
                for galaxy in galaxies.iter_mut() {
                    if galaxy[i] > pos {
                        galaxy[i] += offset;
                    }
                }
    
                pos += offset;
                size[i] += offset;
            }
    
            pos += 1;
        }
    }

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in 0..i {
            sum += (galaxies[i][0] - galaxies[j][0]).abs() + (galaxies[i][1] - galaxies[j][1]).abs();
        }
    }

    sum
}

fn main() {
    let galaxies = parse_input(include_str!("../in.txt"));
    let result_part1 = solve(galaxies.clone(), 1);
    let result_part2 = solve(galaxies, 1000000 - 1);

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}