use itertools::Itertools;

const POSSIBLE: (u32, u32, u32) = (12, 13, 14);

fn main() {
    let input = include_str!("../in.txt")
        .lines()
        .map(|game| game.split(':').next_tuple().unwrap())
        .map(|(label, game)| (label, game.split(';')
            .map(|set| set.split(',')
                .map(|pair| pair.split(' ').filter(|t| !t.is_empty()).map(|t| t.trim()).next_tuple().unwrap())
                .map(|(count, cube_name)| (count.parse::<u32>().unwrap(), cube_name))
                .fold((0, 0, 0), |(r, g, b), (count, cube_name)| match cube_name {
                    "red" => (r + count,g , b),
                    "green" => (r, g + count, b),
                    "blue" => (r, g, b + count),
                    _ => (r, g, b)
                })
            ).fold((0, 0, 0), |(r, g, b), cubes| (r.max(cubes.0), g.max(cubes.1), b.max(cubes.2))))
        );
    
    let result_part1 = input.clone()
        .filter(|(_, (r, g, b))| *r <= POSSIBLE.0 && *g <= POSSIBLE.1 && *b <= POSSIBLE.2)
        .map(|(label, _)| label.split(' ').last().unwrap().parse::<u32>().unwrap())
        .sum::<u32>();

    assert!(result_part1 == 1931);
    println!("part1: {:?}", result_part1);

    let result_part2 = input.map(|(_, (red, green, blue))| red * green * blue).sum::<u32>();

    assert!(result_part2 == 83105);
    println!("part2: {:?}", result_part2);
}