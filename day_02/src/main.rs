use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn is_bounded_monotonic(v: &[i32]) -> bool {
    let steps = v[1..]
        .iter()
        .enumerate()
        .map(|(pos, val)| val - v[pos])
        .collect_vec();

    steps.iter().all(|x| *x >= 1 && *x <= 3) || steps.iter().all(|x| *x >= -3 && *x <= -1)
}

fn main() {
    let lines = INPUT
        .lines()
        .collect_vec()
        .iter()
        .map(|x| {
            x.split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect_vec();

    let lines_one_removed = lines
        .iter()
        .map(|x| {
            (0..x.len())
                .map(|i| [&x[0..i], &x[i + 1..]].concat())
                .collect_vec()
        })
        .collect_vec();

    println!(
        "Part 1: {}",
        lines
            .iter()
            .map(|x| is_bounded_monotonic(x.as_slice()) as i32)
            .sum::<i32>()
    );

    println!(
        "Part 2: {}",
        lines_one_removed
            .iter()
            .map(|x| x.iter().map(|i| is_bounded_monotonic(i)).any(|x| x) as i32)
            .sum::<i32>()
    );
}
