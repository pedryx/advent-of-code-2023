use std::collections::HashSet;
use itertools::Itertools;

fn get_total_cards(cards: &Vec<usize>, cache: &mut Vec<Option<usize>>, card_index: usize) -> usize {
    if cache[card_index] != None {
        return cache[card_index].unwrap();
    }

    let mut total = cards[card_index];
    for i in card_index+1..=card_index+total {
        total += get_total_cards(cards, cache, i);
    }

    cache[card_index] = Some(total);
    total
}

fn main() {
    let start = std::time::Instant::now();

    let cards =include_str!("../in.txt")
        .lines()
        .map(|l| l.split([':', '|']).next_tuple().unwrap())
        .map(|(_, w, n)| (w.split(' ').filter(|t| !t.is_empty()).collect::<HashSet<_>>(), n.split(' ')))
        .map(|(w, n)| n.filter(|t| w.contains(t)).count())
        .collect::<Vec<_>>();

    let result_part1 = cards.iter()
        .filter(|n| **n > 0)
        .map(|n| 1 << *n as u32 - 1)
        .sum::<u32>();

    let mut result_part2 = cards.len();
    let mut cache = vec![None; cards.len()];
    result_part2 += (0..cards.len()).map(|c| get_total_cards(&cards, &mut cache, c)).sum::<usize>();

    println!("time: {:?}", start.elapsed());

    println!("part 1: {}", result_part1);
    println!("part 2: {}", result_part2);
}