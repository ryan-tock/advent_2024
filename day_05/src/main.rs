use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let sections = INPUT.split("\n\n").collect_vec();
    let rules = sections[0].split("\n").collect_vec();
    let updates = sections[1].split("\n").collect_vec();

    let rules = rules
        .iter()
        .map(|x| x.split("|").collect_vec())
        .collect_vec();

    let rules = rules
        .iter()
        .map(|x| (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap()))
        .collect_vec();

    let updates = updates
        .iter()
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut valid_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        let mut works = true;
        for rule in rules.iter() {
            let relevant = update
                .iter()
                .filter(|x| rule.0 == **x || rule.1 == **x)
                .collect_vec();
            if relevant.len() < 2 {
                continue;
            }
            if !(*relevant[0] == rule.0 && *relevant[1] == rule.1) {
                works = false;
                break;
            }
        }
        if works {
            valid_updates.push(update);
        }
    }

    let sum_of_middle = valid_updates
        .iter()
        .map(|x| x[(x.len() - 1) / 2])
        .sum::<i32>();

    println!("Part 1: {}", sum_of_middle);
}
