use iter_tools::Itertools;

type Num = i32;

fn main() {
    let (result_part1, result_part2) = include_str!("../in.txt")
        .lines()
        .map(|l| {
            let mut differences = vec![l.split_whitespace().map(|t| t.parse::<Num>().unwrap()).collect_vec()];

            while differences.last().unwrap().iter().any(|&e| e != 0) {
                differences.push(differences.last()
                    .unwrap()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect_vec()
                );
            }

            let mut last = 0;
            let mut first = 0;
            for sequence in differences.iter().rev().skip(1) {
                last = sequence.iter().last().unwrap() + last;
                first = sequence.iter().next().unwrap() - first;
            }

            (last, first)
        }).reduce(|(l1, f1), (l2, f2)| (l1 + l2, f1 + f2)).unwrap();

    println!("part1: {}", result_part1);
    println!("part2: {}", result_part2);
}
