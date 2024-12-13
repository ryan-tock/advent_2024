use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let sections = INPUT
        .split("\n\n")
        .map(|section| section.split("\n").collect_vec())
        .collect_vec();

    let mut scores: [isize; 2] = [0; 2];
    for section in sections {
        let buttons = section[0..=1]
            .iter()
            .map(|&button| {
                button
                    .split("+")
                    .map(|x| x.split(',').next().unwrap())
                    .skip(1)
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect_tuple::<(isize, isize)>()
                    .unwrap()
            })
            .collect_tuple::<((isize, isize), (isize, isize))>()
            .unwrap();
        let target = section[2]
            .split("=")
            .map(|x| x.split(',').next().unwrap())
            .skip(1)
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple::<(isize, isize)>()
            .unwrap();

        for part in 0..2 {
            let target = if part == 0 {
                target
            } else {
                (target.0 + 10000000000000, target.1 + 10000000000000)
            };
            let det = buttons.0 .0 * buttons.1 .1 - buttons.0 .1 * buttons.1 .0;

            let target_in_basis = (
                target.0 * buttons.1 .1 - target.1 * buttons.1 .0,
                -target.0 * buttons.0 .1 + target.1 * buttons.0 .0,
            );

            if target_in_basis.0 % det == 0 && target_in_basis.1 % det == 0 {
                scores[part] += target_in_basis.0 * 3 / det + target_in_basis.1 / det;
            }
        }
    }
    println!("Part 1: {}", scores[0]);
    println!("Part 2: {}", scores[1]);
}
