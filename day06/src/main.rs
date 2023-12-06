use itertools::Itertools;

type Num = u64;

fn calc_ways(&(time, distance): &(Num, Num)) -> Num {
    (0..=time).filter(|&i| i * (time - i) > distance).count() as Num
}

fn main() {
    let result_part1 = include_str!("../in.txt")
        .lines()
        .map(|l| l.split_whitespace().skip(1).map(|t| t.parse::<Num>().unwrap()))
        .tuples()
        .map(|(t, d)| t.zip(d).fold(1, |c, race| c * calc_ways(&race)))
        .next().unwrap();

    let result_part2 = include_str!("../in.txt")
        .lines()
        .map(|l| l.split_whitespace().skip(1).collect::<String>().parse::<Num>().unwrap())
        .next_tuple().unwrap();
    let result_part2 = calc_ways(&result_part2);

    println!("part 1: {}", result_part1);
    println!("part 2: {}", result_part2);
}