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
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        if is_valid(&update, &rules) {
            valid_updates.push(update);
        } else {
            invalid_updates.push(update);
        }
    }

    let sum_of_middle = valid_updates
        .iter()
        .map(|x| x[(x.len() - 1) / 2])
        .sum::<i32>();

    println!("Part 1: {}", sum_of_middle);

    for i in 0..invalid_updates.len() {
        while !is_valid(&invalid_updates[i], &rules) {
            for rule in rules.iter() {
                let pos1 = invalid_updates[i].iter().position(|&x| x == rule.0);
                let pos2 = invalid_updates[i].iter().position(|&x| x == rule.1);
                if pos1.is_none() || pos2.is_none() {
                    continue;
                }
                let pos1 = pos1.unwrap();
                let pos2 = pos2.unwrap();
                if pos1 > pos2 {
                    invalid_updates[i][pos2] = rule.0;
                    invalid_updates[i][pos1] = rule.1;
                }
            }
        }
    }

    let sum_of_middle = invalid_updates
        .iter()
        .map(|x| x[(x.len() - 1) / 2])
        .sum::<i32>();

    println!("Part 2: {:?}", sum_of_middle);
}

fn is_valid(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for rule in rules.iter() {
        let relevant = update
            .iter()
            .filter(|x| rule.0 == **x || rule.1 == **x)
            .collect_vec();
        if relevant.len() < 2 {
            continue;
        }
        if !(*relevant[0] == rule.0 && *relevant[1] == rule.1) {
            return false;
        }
    }
    true
}
