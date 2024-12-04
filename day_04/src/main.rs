use itertools::{iproduct, Itertools};
const INPUT: &str = include_str!("input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let dirs = iproduct!(-1i32..=1i32, -1i32..=1i32).collect_vec();
    let dirs = dirs.into_iter().filter(|dir| dir != &(0, 0)).collect_vec();

    let mut words = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for dir in &dirs {
                let mut pos = (x as i32, y as i32);
                let mut working_string = String::new();
                loop {
                    if pos.1 < 0 || pos.1 >= grid.len() as i32 {
                        break;
                    }
                    if pos.0 < 0 || pos.0 >= grid[pos.1 as usize].len() as i32 {
                        break;
                    }
                    working_string.push(grid[pos.1 as usize][pos.0 as usize]);

                    if working_string.len() >= 4 {
                        break;
                    }

                    pos.0 += dir.0;
                    pos.1 += dir.1;
                }
                if working_string.as_str() == "XMAS" {
                    words += 1;
                }
            }
        }
    }

    println!("Part 1: {}", words);

    let center_locs = iproduct!(1..grid[0].len() - 1, 1..grid.len() - 1).collect_vec();

    let xmas_forward_forward = center_locs
        .iter()
        .map(|(x, y)| {
            (grid[*y - 1][*x - 1] == 'M'
                && grid[*y - 1][*x + 1] == 'M'
                && grid[*y][*x] == 'A'
                && grid[*y + 1][*x - 1] == 'S'
                && grid[*y + 1][*x + 1] == 'S') as i32
        })
        .sum::<i32>();
    let xmas_forward_backward = center_locs
        .iter()
        .map(|(x, y)| {
            (grid[*y - 1][*x - 1] == 'M'
                && grid[*y - 1][*x + 1] == 'S'
                && grid[*y][*x] == 'A'
                && grid[*y + 1][*x - 1] == 'M'
                && grid[*y + 1][*x + 1] == 'S') as i32
        })
        .sum::<i32>();
    let xmas_backward_forward = center_locs
        .iter()
        .map(|(x, y)| {
            (grid[*y - 1][*x - 1] == 'S'
                && grid[*y - 1][*x + 1] == 'M'
                && grid[*y][*x] == 'A'
                && grid[*y + 1][*x - 1] == 'S'
                && grid[*y + 1][*x + 1] == 'M') as i32
        })
        .sum::<i32>();
    let xmas_backward_backward = center_locs
        .iter()
        .map(|(x, y)| {
            (grid[*y - 1][*x - 1] == 'S'
                && grid[*y - 1][*x + 1] == 'S'
                && grid[*y][*x] == 'A'
                && grid[*y + 1][*x - 1] == 'M'
                && grid[*y + 1][*x + 1] == 'M') as i32
        })
        .sum::<i32>();

    println!(
        "Part 2: {}",
        xmas_forward_forward
            + xmas_forward_backward
            + xmas_backward_forward
            + xmas_backward_backward
    );
}
