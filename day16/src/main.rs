use std::collections::HashSet;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: (isize, isize),
    dir: (isize, isize),
}

fn map_beam(beam: Beam, tile: char) -> Vec<Beam> {
    match (beam, tile) {
        (_, '.') => vec![beam],

        (Beam { pos, dir: (x, y) }, '/') => vec![Beam { pos, dir: ( -y, -x) }],
        (Beam { pos, dir: (x, y) }, '\\') => vec![Beam { pos, dir: ( y,  x) }],

        (Beam { pos, dir: (x, y) }, '-') if y == 0 => vec![Beam { pos, dir: (x, y) }],
        (Beam { pos, dir: (_, y) }, '-') if y != 0 => vec![Beam { pos, dir: (1, 0) }, Beam { pos, dir: (-1, 0) }],
        (Beam { pos, dir: (x, y) }, '|') if x == 0 => vec![Beam { pos, dir: (x, y) }],
        (Beam { pos, dir: (x, _) }, '|') if x != 0 => vec![Beam { pos, dir: (0, 1) }, Beam { pos, dir: (0, -1) }],

        _ => panic!("Invalid beam to tile mapping."),
    }
}

fn calc_energized_tiles(layout: &Vec<Vec<char>>, start_beam: Beam) -> usize {
    let mut energized_tiles = HashSet::new();
    let mut states = HashSet::new();

    let horizontal_range = 0..layout[0].len() as isize;
    let vertical_range = 0..layout.len() as isize;

    let mut beams = map_beam(start_beam, layout[start_beam.pos.1 as usize][start_beam.pos.0 as usize]);
    while let Some(mut current_beam) = beams.pop() {
        if !states.insert(current_beam) {
            continue;
        }

        energized_tiles.insert(current_beam.pos);

        current_beam.pos.0 += current_beam.dir.0;
        current_beam.pos.1 += current_beam.dir.1;

        if !(horizontal_range.contains(&current_beam.pos.0) && vertical_range.contains(&current_beam.pos.1)) {
            continue;
        }

        let tile = layout[current_beam.pos.1 as usize][current_beam.pos.0 as usize];
        beams.extend(map_beam(current_beam, tile));
    }

    energized_tiles.len()
}

fn main() {
    let layout = include_str!("../in.txt")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    
    let result_part1 = calc_energized_tiles(&layout, Beam { pos: (0, 0), dir: (1, 0) });

    let mut result_part2 = 0;
    for x in 0..layout[0].len() as isize {
        let beam = Beam { pos: (x, 0), dir: (0, 1) };
        result_part2 = result_part2.max(calc_energized_tiles(&layout, beam));

        let beam = Beam { pos: (x, layout.len() as isize - 1), dir: (0, -1) };
        result_part2 = result_part2.max(calc_energized_tiles(&layout, beam));
    }    
    for y in 0..layout.len() as isize {
        let beam = Beam { pos: (0, y), dir: (1, 0) };
        result_part2 = result_part2.max(calc_energized_tiles(&layout, beam));

        let beam = Beam { pos: (layout[0].len() as isize - 1, y), dir: (-1, 0) };
        result_part2 = result_part2.max(calc_energized_tiles(&layout, beam));
    }
    
    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}