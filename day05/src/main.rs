use itertools::Itertools;

type Num = u64;
type Interval = (Num, Num);
type Mapping = (Interval, Num);
type MappingList = Vec<Mapping>;

const INTERVAL_MIN: Num = 0;
const INTERVAL_MAX: Num = Num::MAX;

trait IntervalOp<T> {
    fn len(&self) -> Num;
    fn try_intersect(&self, other: &Interval) -> Option<Interval>;
}

impl IntervalOp<Interval> for Interval {
    fn try_intersect(&self, other: &Interval) -> Option<Interval> {
        let start = self.0.max(other.0);
        let end = self.1.min(other.1);
    
        if end < start { None } else { Some((start, end)) }
    }

    fn len(&self) -> Num { self.1 - self.0 }
}

fn solve(seeds: &[Interval], layers: &[MappingList], layer_index: usize, current: Interval) -> Option<Num> {
    if layer_index == 0 {
        return seeds.iter().find_map(|interval| interval.try_intersect(&current).map(|i| i.0));
    }

    let layer = &layers[layer_index - 1];
    let left_index = layer.binary_search_by_key(&current.0, |&((start, _), _)| start)
        .unwrap_or_else(|i| i.saturating_sub(1));
    let right_index = layer.binary_search_by_key(&current.1, |&((start, _), _)| start)
        .unwrap_or_else(|i| i.saturating_sub(1));

    for i in left_index..=right_index {
        let mapping = layer[i];
        let intersection = mapping.0.try_intersect(&current).unwrap();
        let mapped_start = mapping.1 + (intersection.0 - mapping.0.0);
        let mapped = (mapped_start, mapped_start + intersection.len());
        if let Some(location) = solve(seeds, layers, layer_index - 1, mapped) {
            return  Some(location);
        }
    }

    None
}

fn map_seed(layers: &Vec<MappingList>, seed: Num) -> Num {
    let mut seed = seed;

    for layer in layers {
        for &((dst_start, dst_end), src_start)in layer {
            if src_start <= seed && seed <= src_start + (dst_end - dst_start) {
                seed = dst_start + (seed - src_start);
                break;
            }
        }
    }

    seed
}

fn preprocess_layers<It1, It2>(layers: It1) -> Vec<MappingList> 
where 
    It1: Iterator<Item = It2>,
    It2: Iterator<Item = ((Num, Num), Num)>,
{
    layers.map(|layer| {
        let mut new_layer = Vec::new();
        let mut current = INTERVAL_MIN;

        for interval in layer {
            if current < interval.0.0 {
                new_layer.push(((current, interval.0.0 - 1), current));
            }

            new_layer.push(interval);
            current = interval.0.1 + 1;
        }

        new_layer.push(((current, INTERVAL_MAX), current));
        new_layer
    }).collect()
}

fn main() {
    let mut input = include_str!("../in.txt")
        .split("\n\n")
        .map(|t| t.split(':').last().unwrap().split_whitespace().map(|n| n.parse::<Num>().unwrap()));

    let seeds_part1 = input.clone().next().unwrap();
    let seeds_part2 = input.next().unwrap()
        .tuples()
        .map(|(start, len)| (start, start + len - 1))
        .sorted_by_key(|&(start, _)| start)
        .collect_vec();

    let layers = input.map(|m| m.tuples()
        .map(|(dst, src, len)| ((dst, dst + len - 1), src))
        .sorted_by_key(|&((start, _), _)| start));
    let layers = preprocess_layers(layers);

    let result_part1 = seeds_part1.map(|s| map_seed(&layers, s)).min().unwrap();

    let last_layer = layers.len() - 1;
    let result_part2 = layers[last_layer].iter()
        .find_map(|&(interval, _)| 
            solve(&seeds_part2, &layers, last_layer - 1, interval).map(|loc| map_seed(&layers, loc))
        ).unwrap();

    println!("part 1: {}", result_part1);
    println!("part 2: {}", result_part2);
}