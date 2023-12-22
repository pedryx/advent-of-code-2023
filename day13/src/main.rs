fn is_reflection(input: &Vec<Vec<char>>, row: usize, smudge: bool) -> bool {
    let mut check = false;
    let mut smudge = !smudge;
    
    for i in 0..=row {
        if row + i + 1 == input.len() {
            break;
        }
        check = true;
        
        for j in 0..input[0].len() {
            if input[row - i][j] != input[row + i + 1][j] {
                if !smudge {
                    smudge = true;
                    continue;
                }
                
                return false;
            }
        }
    }
    
    check && smudge
}

fn find_reflection(input: &Vec<Vec<char>>, smudge: bool) -> Option<usize> {
    (0..input.len())
        .filter(|&i| is_reflection(input, i, smudge))
        .next()
        .map(|i| i + 1)
}

fn transpose(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|line| line[i]).collect::<Vec<_>>())
        .collect()
}

fn solve(input: &Vec<Vec<char>>, smudge: bool) -> usize {
    find_reflection(input, smudge)
        .map_or_else(|| find_reflection(&transpose(input), smudge).unwrap_or(0), |row| row * 100)
}

fn main() {
    let input = include_str!("../in.txt")
        .split("\n\n")
        .map(|t| t.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>());

    let result_part1 = input.clone().map(|t| solve(&t, false)).sum::<usize>();
    let result_part2 = input.map(|t| solve(&t, true)).sum::<usize>();

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}