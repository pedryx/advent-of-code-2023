use std::collections::HashMap;
use itertools::Itertools;

type Num = u64;

fn can_place(d1: &[char], d2: &[usize]) -> bool {
    (0..d2[0]).all(|i| i < d1.len() && d1[i] != '.') && (d2[0] >= d1.len() || d1[d2[0]] != '#')
}

fn is_correct(d1: &[char], d2: &[usize]) -> bool {
    (d2[0] + 1..d1.len()).all(|i| d1[i] != '#')
}

fn solve_line(d1: &[char], d2: &[usize], cache: &mut HashMap<(usize, usize), Num>) -> Num {
    if d1.len() == 0 {
        return 0;
    }
   
    let cache_key = (d1.len(), d2.len());
    if let Some(&sum) = cache.get(&cache_key) {
        return sum;
    }

    let mut sum = 0;
    if can_place(d1, d2) {
        if d2.len() == 1 {
            if is_correct(d1, d2) {
                sum += 1;
            }
        }
        else if d2[0] + 1 < d1.len() {
            sum += solve_line(&d1[d2[0] + 1..], &d2[1..], cache);
        }
    }
    if d1[0] != '#' {
        sum += solve_line(&d1[1..], d2, cache);
    }

    cache.insert(cache_key, sum);
    sum
}

fn solve(input: &str, repeat: usize) -> Num {
    let input = input.lines()
        .map(|l| l.split_whitespace().next_tuple().unwrap())
        .map(|(d1, d2)| (
            d1.chars()
                .chain(std::iter::repeat(['?'].into_iter().chain(d1.chars())).take(repeat - 1).flatten())
                .collect_vec(), 
            std::iter::repeat(d2.split(',').map(|n| n.parse::<usize>().unwrap())).take(repeat).flatten().collect_vec()
        ));

    let mut sum = 0;
    for (d1, d2) in input {
        sum += solve_line(&d1, &d2, &mut HashMap::new());
    }

    sum
}

fn main() {
    let result_part1 = solve(include_str!("../in.txt"), 1);
    let result_part2 = solve(include_str!("../in.txt"), 5);

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}