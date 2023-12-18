use itertools::Itertools;
use std::i64;

fn shoelace(points: &Vec<(i64, i64)>) -> i64 {
    let mut area = 0;
    for i in 0..points.len() {
        let current = points[i];
        let next = points[(i + 1) % points.len()];

        area += current.0 * next.1 - next.0 * current.1;
    }

    area / 2
}

fn parse_part1<It>(lines: It) -> (Vec<(i64, i64)>, i64)
    where It : Iterator<Item = (&'static str, &'static str, &'static str)>
{
    let mut points = Vec::new();

    let mut pos = (0, 0);
    let mut circumference = 0;
    for (direction, amount, _) in lines {
        let amount = amount.parse::<i64>().unwrap();
        match direction {
            "U" => pos.1 -= amount,
            "D" => pos.1 += amount,
            "L" => pos.0 -= amount,
            "R" => pos.0 += amount,
            _ => panic!("Invalid direction."),
        }

        circumference += amount;
        points.push(pos);
    }

    (points, circumference)
}

fn parse_part2<It>(lines: It) -> (Vec<(i64, i64)>, i64)
    where It : Iterator<Item = (&'static str, &'static str, &'static str)>
{
    let mut points = Vec::new();

    let mut pos = (0, 0);
    let mut circumference = 0;
    for (_, _, hex) in lines {
        let amount = i64::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();
        let direction = &hex[hex.len() - 2..hex.len() - 1];

        match direction {
            "3" => pos.1 -= amount,
            "1" => pos.1 += amount,
            "2" => pos.0 -= amount,
            "0" => pos.0 += amount,
            _ => panic!("Invalid direction."),
        }

        circumference += amount;
        points.push(pos);
    }

    (points, circumference)
}

fn main() {
    let input = include_str!("../in.txt")
        .lines()
        .map(|l| l.split_whitespace().next_tuple().unwrap());

    let (points_part1, circumference_part1) = parse_part1(input.clone());
    let area_part1 = shoelace(&points_part1);
    println!("part1: {}", area_part1 + (circumference_part1 / 2) + 1);

    let (points_part2, circumference_part2) = parse_part2(input);
    let area_part2 = shoelace(&points_part2);
    println!("part2: {}", area_part2 + (circumference_part2 / 2) + 1);
}