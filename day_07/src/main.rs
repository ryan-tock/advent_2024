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

    let mut calibration_count: i64 = 0;

    for line in lines.iter() {
        let operators = line.1.len() - 1;

        let mut works = false;

        for state in 0usize..2usize.pow(operators as u32) {
            let mut result = line.1[0];

            for num in 1usize..=operators {
                if (state >> (num - 1)) % 2 == 0 {
                    result += line.1[num];
                } else {
                    result *= line.1[num];
                }
            }

            if result == line.0 {
                works = true;
                break;
            }
        }

        if works {
            calibration_count += line.0;
        }
    }

    println!("Part 1: {}", calibration_count);
}
