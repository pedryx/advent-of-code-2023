use std::collections::HashMap;
use itertools::Itertools;

type Num = u32;

fn solve(card_to_num: HashMap<char, Num>, jokers_enabled: bool) -> Num {
    include_str!("../in.txt")
        .lines()
        .map(|line|  {
            let (hand, bid) = line.split_whitespace().next_tuple().unwrap();
            let hand_numbers = hand.chars()
                .map(|c| if c.is_numeric() { c as Num - '0' as Num } else { card_to_num[&c] })
                .next_tuple::<(_, _, _, _, _)>().unwrap();
            let bid = bid.parse::<Num>().unwrap();

            let (values, jokers) = hand.chars().fold((HashMap::new(), 0), |(mut map, mut jokers), c| {
                if jokers_enabled && c == 'J' {
                    jokers += 1;
                }
                else {
                    *map.entry(c).or_insert(0) += 1;
                }
                (map, jokers)
            });

            let mut values = values.values().sorted().rev().collect::<Vec<_>>();
            values.push(&0);

            let score = if values[0] + jokers >= 5 { 6 }
            else if values[0] + jokers >= 4 { 5 }
            else if values[0] + jokers >= 3 && values[1] >= &2 { 4 }
            else if values[0] >= &3 && values[1] + jokers >= 2 { 4 }
            else if values[0] + jokers / 2 >= 3 && values[1] + jokers / 2 >= 2 { 4 }
            else if values[0] + jokers >= 3 { 3 }
            else if values[0] + jokers >= 2 && values[1] >= &2 { 2 }
            else if values[0] >= &2 && values[1] + jokers >= 2 { 2 }
            else if values[0] + jokers / 2 >= 2 && values[1] + jokers / 2 >= 2 { 2 }
            else if values[0] + jokers >= 2 { 1 }
            else { 0 };

            (hand_numbers, bid, score)
        }).sorted_by_key(|&(hand_numbers, _, score)| (score, hand_numbers))
        .enumerate()
        .map(|(rank, (_, bid, _))| bid * (rank as Num + 1))
        .sum::<Num>()
}

fn main() {
    let card_to_num_part1 = HashMap::from([
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let result_part1 = solve(card_to_num_part1, false);

    let card_to_num_part2 = HashMap::from([
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
        ('J', 1),
    ]);
    let result_part2 = solve(card_to_num_part2, true);

    println!("part2: {:?}", result_part1);
    println!("part1: {:?}", result_part2);
}