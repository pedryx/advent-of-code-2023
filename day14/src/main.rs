type Grid = Vec<Vec<char>>;

fn rotate(input: &Grid) -> Grid {
    (0..input[0].len())
        .map(|i| input.iter().map(|line| line[i]).rev().collect())
        .collect()
}

fn calc_load(input: &Grid) -> usize {
    input.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().filter(|&&c| c == 'O').map(move |_| input.len() - y))
        .sum()
}

fn move_rocks(input: &mut Grid) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'O' {
                let mut y = y;
                input[y][x] = '.';

                while y > 0 && input[y - 1][x] == '.' {
                    y -= 1;
                }

                input[y][x] = 'O';
            }
        }
    }
}

fn solve_part1(mut input: Grid) -> usize {
    move_rocks(&mut input);
    calc_load(&input)
}

fn solve_part2(mut input: Grid) -> usize {
    const PATTERN_LEN: usize = 2;
    const START_OFFSET: usize = 110;
    const CYCLE_COUNT:usize = 1000000000;

    let mut results = Vec::new();
    let mut cycle = 0;    

    for i in 0..CYCLE_COUNT {
        for _ in 0..4 {
            move_rocks(&mut input);
            input = rotate(&input);
        }

        if i < START_OFFSET {
            continue;
        }

        let result = calc_load(&input);
        results.push(result);


        if let Some(pos) = results.iter().position(|&item| item == result) {
            if pos != 0 && pos != results.len() - 1 {
                if (1..PATTERN_LEN).all(|i| results[results.len() - i] == results[pos - i + 1]) {
                    cycle = pos;
                    break;
                }
            }
        }
    }

    let repeat = (results.len() - 1) - cycle;
    let pos = ((CYCLE_COUNT - cycle - START_OFFSET)) % (repeat);

    results[pos]
}

fn main() {
    let input = include_str!("../in.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result_part1 = solve_part1(input.clone());
    let result_part2 = solve_part2(input);

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}