use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut entries: Vec<(i32, i32)> = Vec::new();
    for i in INPUT.lines() {
        let mut iter = i.split("   ");

        let num1 = iter.next().unwrap().parse::<i32>().unwrap();
        let num2 = iter.next().unwrap().parse::<i32>().unwrap();

        entries.push((num1, num2));
    }

    let entries2 = INPUT
        .lines()
        .map(|x| {
            let mut iter = x.split("   ");
            let num1 = iter.next().unwrap().parse::<i32>().unwrap();
            let num2 = iter.next().unwrap().parse::<i32>().unwrap();
            (num1, num2)
        })
        .collect::<Vec<_>>();

    let entries3 = INPUT
        .lines()
        .map(|x| {
            x.split("   ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let column1 = entries3.iter().map(|x| x.0).sorted().collect_vec();
    let column2 = entries3.iter().map(|x| x.1).sorted().collect_vec();

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

    // let mut foo: String = String::new();
    // let bar = &mut foo;
    // fn xyz(i: String) -> String {
    //     i.to_uppercase()
    // }
    // let bar2 = bar.clone();
    // println!("{}", foo);
    // xyz(bar2);

    println!("Part 1: {}", difference);

    println!("Part 2: {}", similarity_score);
}
