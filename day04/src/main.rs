use std::collections::HashSet;
use itertools::Itertools;

fn get_total_cards(cards: &Vec<usize>, cache: &mut Vec<Option<usize>>, card_index: usize) -> usize {
    if let Some(total) = cache[card_index] {
        return total;
    }

    let total = cards[card_index] + (card_index + 1..=card_index + cards[card_index])
        .map(|i| get_total_cards(cards, cache, i))
        .sum::<usize>();

    cache[card_index] = Some(total);
    total
}

fn main() {
    let time = std::time::Instant::now();

    let cards =include_str!("../in.txt")
        .lines()
        .map(|l| l.split([':', '|']).next_tuple().unwrap())
        .map(|(_, w, n)| (w.split_whitespace().collect::<HashSet<_>>(), n.split_whitespace()))
        .map(|(w, n)| n.filter(|t| w.contains(t)).count())
        .collect::<Vec<_>>();

    let result_part1 = cards.iter()
        .filter(|&&n| n > 0)
        .map(|&n| 1 << n as u32 - 1)
        .sum::<u32>();

    let mut result_part2 = cards.len();
    let mut cache = vec![None; cards.len()];
    result_part2 += (0..cards.len()).map(|c| get_total_cards(&cards, &mut cache, c)).sum::<usize>();

    println!("time: {:?}", time.elapsed());
    println!("part 1: {}", result_part1);
    println!("part 2: {}", result_part2);
}