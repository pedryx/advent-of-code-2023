use itertools::Itertools;

type Distance = u16;
type Coord = i16;

#[derive(Clone, Copy)]
struct Tile {
    // up right down left
    directions: u8,
    visited: bool,
    came_from: (Coord, Coord),
    on_loop: bool,
}

const MASK_UP: u8    = 0b1000; 
const MASK_RIGHT: u8 = 0b0100; 
const MASK_DOWN: u8  = 0b0010; 
const MASK_LEFT: u8  = 0b0001; 
const NEIGHBORS: &[((Coord, Coord), u8, u8)] = &[
    ((0, -1), MASK_UP, MASK_DOWN), 
    ((1, 0), MASK_RIGHT, MASK_LEFT), 
    ((0, 1), MASK_DOWN, MASK_UP), 
    ((-1, 0), MASK_LEFT, MASK_RIGHT)
];

fn parse_input(input: &'static str) -> (Vec<Vec<Tile>>, (Coord, Coord)) {
    let mut start = (0, 0);

    let network = input.lines().enumerate().map(|(y, l)| 
        l.chars().enumerate().map(|(x, c)| {
            let packed = match c {
                '|' => 0b1010,
                '-' => 0b0101,
                'L' => 0b1100,
                'J' => 0b1001,
                '7' => 0b0011,
                'F' => 0b0110,
                '.' => 0b0000,
                'S' => { start = (x as Coord, y as Coord); 0b1111 },
                _   => panic!("Invalid tile character."),
            };
            
            Tile { directions: packed, visited: false, came_from: (0, 0), on_loop: false }
        }).collect_vec()
    ).collect_vec();

    (network, start)
}

fn find_loop(network: &mut Vec<Vec<Tile>>, start: (Coord, Coord)) {
    let mut stack = vec![start];
    let mut loop_start = (0, 0);
    let mut loop_end = (0, 0);

    'dfs: while let Some(current) = stack.pop() {
        let current_tile = network[current.1 as usize][current.0 as usize];

        for (neighbor, mask_from, mask_to) in NEIGHBORS {
            let neighbor = (current.0 + neighbor.0, current.1 + neighbor.1);

            if neighbor.0 < 0 || neighbor.0 >= network[0].len() as Coord 
                || neighbor.1 < 0 || neighbor.1 >= network.len() as Coord
            {
                continue;
            }

            let neighbor_tile = &mut network[neighbor.1 as usize][neighbor.0 as usize];

            if !(current_tile.directions & mask_from > 0 && neighbor_tile.directions & mask_to > 0) {
                continue;
            }

            if neighbor_tile.visited && current_tile.came_from != neighbor {
                loop_start = current;
                loop_end = neighbor;
                break 'dfs;         
            }

            if !neighbor_tile.visited {
                neighbor_tile.visited = true;
                neighbor_tile.came_from = current;
                stack.push(neighbor);
            }
        }
    }

    let mut current = loop_start;
    while current != start {
        let tile = &mut network[current.1 as usize][current.0 as usize];

        tile.on_loop = true;
        current = tile.came_from;
    }

    network[loop_end.1 as usize][loop_end.0 as usize].on_loop = true;
    network[start.1 as usize][start.0 as usize].on_loop = true;
}

fn remap_start(network: &mut Vec<Vec<Tile>>, start: (Coord, Coord)) {
    let mut directions = 0;

    for (neighbor, mask_from, mask_to) in NEIGHBORS {
        let coord = (start.0 + neighbor.0, start.1 + neighbor.1);

        if coord.0 < 0 || coord.0 >= network[0].len() as Coord 
            || coord.1 < 0 || coord.1 >= network.len() as Coord
        {
            continue;
        }

        let tile = &mut network[coord.1 as usize][coord.0 as usize];
        if tile.on_loop && tile.directions & mask_to > 0 {
            directions |= mask_from;
        }
    }

    network[start.1 as usize][start.0 as usize].directions = directions;
}

fn count_tiles(network: &mut Vec<Vec<Tile>>) -> (Distance, Distance) {
    let mut inner_tiles_count: Distance = 0;
    let mut on_loop = false;
    let mut loop_encounters = 0;
    let mut from_up = false;
    let mut loop_tiles_count = 0;

    for y in 0..network.len() as Coord {
        for x in 0..network[0].len() as Coord {
            let start = (x, y);
            let tile = &mut network[start.1 as usize][start.0 as usize];

            if tile.on_loop {
                loop_tiles_count += 1;
            }

            if tile.on_loop {
                if tile.directions & MASK_UP > 0 && tile.directions & MASK_DOWN > 0 {
                    loop_encounters += 1;
                    continue;
                }

                if !(tile.directions & MASK_UP > 0 || tile.directions & MASK_DOWN > 0) {
                    continue;
                }

                if !on_loop {
                    from_up = tile.directions & MASK_UP > 0;
                    on_loop = true;
                    continue;
                }

                if from_up != (tile.directions & MASK_UP > 0) {
                    loop_encounters += 1;
                }

                on_loop = false;
                continue;
            }

            if loop_encounters % 2 == 1 {
                inner_tiles_count += 1;
            }
        }

        on_loop = false;
        loop_encounters = 0;
        from_up = false;
    }

    (loop_tiles_count, inner_tiles_count)
}

fn main() {
    let (mut network, start) = parse_input(include_str!("../in.txt"));
    find_loop(&mut network, start);
    remap_start(&mut network, start);
    let (loop_tiles_count, inner_tiles_count) = count_tiles(&mut network);

    println!("part1: {}", (loop_tiles_count + 1) / 2);
    println!("part2: {}", inner_tiles_count);
}