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

    let mut calibration_count: i64 = 0;

    for line in lines.iter() {
        let operators = line.1.len() - 1;

        let mut works = false;

        for state in 0usize..3usize.pow(operators as u32) {
            let mut result = line.1[0];

            for num in 1usize..=operators {
                let operator_num = (state / 3usize.pow((num - 1) as u32)) % 3;
                if operator_num == 0 {
                    result += line.1[num];
                } else if operator_num == 1 {
                    result *= line.1[num];
                } else {
                    result *= 10usize.pow(line.1[num].to_string().len() as u32) as i64;
                    result += line.1[num];
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

    println!("Part 2: {}", calibration_count);
}
