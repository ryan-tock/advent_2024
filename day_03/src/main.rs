use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let disables = INPUT.split("don't()").collect_vec();
    let active_code = disables[1..]
        .iter()
        .map(|x| x.split("do()").collect_vec()[1..].join(""))
        .collect_vec();

    let active_code = disables[0].to_owned() + &active_code.join("");

    let mut total: i32 = 0;
    for (_, [num1, num2]) in re.captures_iter(INPUT).map(|captures| captures.extract()) {
        total += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
    }
    println!("Part 1: {}", total);

    let mut total: i32 = 0;
    for (_, [num1, num2]) in re
        .captures_iter(&active_code)
        .map(|captures| captures.extract())
    {
        total += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
    }
    println!("Part 2: {}", total);
}
