use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let lines = INPUT
        .lines()
        .map(|line| line.split(": ").collect_tuple::<(&str, &str)>().unwrap())
        .collect_vec();

    let lines = lines
        .iter()
        .map(|line| {
            (
                line.0.parse::<i64>().unwrap(),
                line.1
                    .split(" ")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    use std::time::Instant;
    let start = Instant::now();

    let calibration_result = lines
        .iter()
        .map(|(target, nums)| target * is_vaild(*target, nums, false) as i64)
        .sum::<i64>();
    println!("Part 1: {}", calibration_result);

    let part1 = start.elapsed().as_millis();

    let calibration_result = lines
        .iter()
        .map(|(target, nums)| target * is_vaild(*target, nums, true) as i64)
        .sum::<i64>();
    println!("Part 2: {}", calibration_result);

    let part2 = start.elapsed().as_millis();

    println!("Part 1 time: {} ms", part1);
    println!("Part 2 time: {} ms", part2 - part1);
    println!("Total time: {} ms", part2);
}

fn is_vaild(target: i64, numbers: &Vec<i64>, concat: bool) -> bool {
    let mut possible_results: Vec<i64> = Vec::new();
    possible_results.push(numbers[0]);

    for num_index in 1..numbers.len() {
        let mut new_results: Vec<i64> = Vec::new();
        for i in 0..possible_results.len() {
            let mut new: [i64; 3] = [0; 3];
            new[0] = possible_results[i] + numbers[num_index];
            new[1] = possible_results[i] * numbers[num_index];
            if concat {
                new[2] = (possible_results[i].to_string()
                    + numbers[num_index].to_string().as_str())
                .parse::<i64>()
                .unwrap();
            }
            for x in 0..(2 + concat as usize) {
                if new[x] == target {
                    return true;
                }
                if new[x] < target {
                    new_results.push(new[x]);
                }
            }
        }

        possible_results = new_results
    }

    false
}
