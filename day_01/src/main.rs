use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let entries = INPUT
        .lines()
        .map(|x| {
            x.split("   ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let column1 = entries.iter().map(|x| x.0).sorted().collect_vec();
    let column2 = entries.iter().map(|x| x.1).sorted().collect_vec();

    let mut occurrences = HashMap::new();
    for i in column2.iter() {
        *occurrences.entry(*i).or_insert(0) += 1;
    }

    let similarity_score = column1
        .iter()
        .map(|x| x * occurrences.get(x).copied().unwrap_or_default())
        .sum::<i32>();

    let zipped = column1.into_iter().zip(column2.into_iter()).collect_vec();

    let difference = zipped.iter().map(|x| (x.0 - x.1).abs()).sum::<i32>();

    println!("Part 1: {}", difference);

    println!("Part 2: {}", similarity_score);
}
