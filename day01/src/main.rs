use aho_corasick::AhoCorasick;

fn solve_part1() -> usize
{
    let values = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let ac = AhoCorasick::new(values).unwrap();

    include_str!("../in1.txt")
        .lines()
        .map(|l| ac.find_overlapping_iter(l)
            .map(|m| m.pattern().as_usize())
            .peekable()
        ).map(|mut l| l.peek().unwrap() * 10 + l.last().unwrap())
        .sum()
}

fn solve_part2() -> usize
{
    let values = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let ac = AhoCorasick::new(values).unwrap();

    include_str!("../in1.txt")
        .lines()
        .map(|l| ac.find_overlapping_iter(l)
            .map(|m| m.pattern().as_usize())
            .map(|id| if id >= values.len() / 2 { id - values.len() / 2 + 1 } else { id + 1 })
            .peekable()
        ).map(|mut l| l.peek().unwrap() * 10 + l.last().unwrap())
        .sum()
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}