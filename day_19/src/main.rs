use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let sections = INPUT.split("\n\n").collect_vec();
    let mut towels = sections[0].split(", ").collect_vec();
    let patterns = sections[1].lines();
    let mut valid = HashMap::new();

    towels.sort_by(|&a, &b| a.len().cmp(&b.len()));

    for (i, &towel) in towels.iter().enumerate() {
        let work_count = works(towel, &Vec::from(&towels[0..i]), &mut valid);
        valid.insert(towel.to_owned(), work_count + 1);
    }

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for pattern in patterns {
        let amount_works = works(pattern, &towels, &mut valid);
        if amount_works > 0 {
            total_part1 += 1;
        }
        total_part2 += amount_works;
    }

    println!("Part 1: {}", total_part1);
    println!("Part 2: {}", total_part2);
}

fn works(pattern: &str, towels: &Vec<&str>, valid: &mut HashMap<String, usize>) -> usize {
    if let Some(v) = valid.get(pattern) {
        return *v;
    }

    let mut new_total = 0;
    for &towel in towels {
        if let Some(new_pattern) = pattern.strip_prefix(towel) {
            new_total += works(new_pattern, towels, valid);
        }
    }
    if new_total > 0 {
        valid.insert(pattern.to_owned(), new_total);
    }
    new_total
}
