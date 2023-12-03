use std::collections::HashMap;
use regex::Regex;

type Num = u32;
type Point = (usize, usize);
type Schematic = Vec<&'static str>;
type Gears = HashMap<(usize, usize), (Num, usize)>;

fn process_neighborhood(schematic: &Schematic, gears: &mut Gears, start: Point, end: Point, num: Num) -> bool {
    for y in start.1..=end.1.min(schematic.len() - 1) {
        for (x, c) in schematic[y].chars().enumerate().skip(start.0).take(end.0 - start.0 + 1) {
            if (y == start.1 || y == end.1 || x == start.0 || x == end.0) && (c != '.' && !c.is_numeric())
            {
                if c == '*' {
                    let (ratio, count) = gears.entry((x, y)).or_insert((1, 0));

                    *ratio *= num;
                    *count += 1;
                }

                return true
            }
        }
    }

    false
}

fn main() {
    let schematic = include_str!("../in.txt").lines().collect::<Vec<_>>();
    let pattern = Regex::new(r"\d+").unwrap();
    let mut gears = HashMap::new();
    let mut result_part1 = 0;

    for (y, line) in schematic.iter().enumerate() {
        for m in pattern.find_iter(line) {
            let num = m.as_str().parse().unwrap();
            let start = (m.start().saturating_sub(1), y.saturating_sub(1));
            let end = (m.end(), y + 1);

            if process_neighborhood(&schematic, &mut gears, start, end, num) {
                result_part1 += num;
            }
        }
    }

    let result_part2 = gears.values()
        .filter(|(_, count)| *count >= 2)
        .map(|(ratio, _)| ratio)
        .sum::<Num>();

    println!("part 1: {}", result_part1);
    println!("part 2: {}", result_part2);
}