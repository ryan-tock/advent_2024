use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let initial = INPUT
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();

    let mut unique = HashMap::new();

    for num in initial {
        *unique.entry(num).or_insert(0) += 1usize;
    }

    for _iteration in 0..25 {
        unique = iterate(&unique);
    }

    println!("Part 1: {:?}", unique.values().sum::<usize>());

    for _iteration in 0..50 {
        unique = iterate(&unique);
    }

    println!("Part 2: {:?}", unique.values().sum::<usize>());
}

fn iterate(unique: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_unique = HashMap::new();
    for &entry in unique.keys() {
        if entry == 0 {
            *new_unique.entry(1).or_insert(0) += unique.get(&entry).unwrap();
            continue;
        }
        let power = entry.ilog10() as usize;
        if power % 2 == 1 {
            let sig_digit = 10usize.pow(((power + 1) / 2) as u32);
            let left = entry / sig_digit;
            let right = entry % sig_digit;

            *new_unique.entry(left).or_insert(0) += unique.get(&entry).unwrap();
            *new_unique.entry(right).or_insert(0) += unique.get(&entry).unwrap();
            continue;
        }

        *new_unique.entry(entry * 2024).or_insert(0) += unique.get(&entry).unwrap();
    }
    new_unique
}
