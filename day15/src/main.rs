use itertools::Itertools;

fn calc_hash(s: &str) -> usize {
    s.chars().fold(0, |value, c| ((value + c as usize) * 17) % 256)
}

fn main() {
    let mut boxes: Vec<Vec<(&str, &str)>> = vec!(Vec::new(); 256);
    let input = include_str!("../in.txt").split(',');

    let result_part1 = input.clone().map(calc_hash).sum::<usize>();

    for step in input {
        if step.contains('-') {
            let label = &step[..step.len() - 1];
            let hash = calc_hash(label);

            if let Some(index) = boxes[hash].iter().position(|&(l, _)| l == label) {
                boxes[hash].remove(index);
            }
        }
        // Assuming step contains '='.
        else {
            let (label, focus) = step.split('=').next_tuple().unwrap();
            let hash = calc_hash(label);

            if let Some(index) = boxes[hash].iter().position(|&(l, _)| l == label) {
                boxes[hash][index].1 = focus;
            }
            else {
                boxes[hash].push((label, focus));
            }
        }
    }

    let result_part2 = boxes.iter()
        .enumerate()
        .flat_map(|(box_num, lenses)| lenses.iter()
            .enumerate()
            .map(move |(slot, (_, focus))| (box_num + 1) * (slot + 1) * focus.parse::<usize>().unwrap())
        ).sum::<usize>();

    println!("part1: {}", result_part1);
    println!("part1: {}", result_part2);
}