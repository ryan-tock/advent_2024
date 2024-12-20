use itertools::{iproduct, Itertools};
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut start = (0usize, 0usize);
    let mut end = (0usize, 0usize);

    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == 'S' {
                start = (x, y);
            }
            if c == 'E' {
                end = (x, y);
            }
        }
    }

    let mut tiles_to_end = HashMap::new();
    tiles_to_end.insert(end, 0usize);
    let mut curr = (end, 0);

    while curr.0 != start {
        for &relative in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let (x, y) = (
                curr.0 .0 as isize + relative.0,
                curr.0 .1 as isize + relative.1,
            );
            let (x, y) = (x as usize, y as usize);
            if grid[y][x] == '#' {
                continue;
            }
            if tiles_to_end.contains_key(&(x, y)) {
                continue;
            }
            curr = ((x, y), curr.1 + 1);
            tiles_to_end.insert(curr.0, curr.1);
        }
    }

    let mut cheats = HashMap::new();

    for (x, y) in iproduct!(0..grid[0].len(), 0..grid.len()) {
        for cheat_positions in [
            (2, 0),
            (-2, 0),
            (0, 2),
            (0, -2),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            if grid[y][x] == '#' {
                continue;
            }
            let (x_cheat, y_cheat) = (
                x as isize + cheat_positions.0,
                y as isize + cheat_positions.1,
            );
            if x_cheat < 0 || x_cheat >= grid[0].len() as isize {
                continue;
            }
            if y_cheat < 0 || y_cheat >= grid.len() as isize {
                continue;
            }
            let (x_cheat, y_cheat) = (x_cheat as usize, y_cheat as usize);
            if grid[y_cheat][x_cheat] == '#' {
                continue;
            }
            let savings = *tiles_to_end.get(&(x_cheat, y_cheat)).unwrap() as isize
                - *tiles_to_end.get(&(x, y)).unwrap() as isize
                - 2;

            if savings > 0 {
                cheats.insert(((x, y), (x_cheat, y_cheat)), savings as usize);
            }
        }
    }

    println!(
        "Part 1: {:?}",
        cheats.values().filter(|&&v| v >= 100).count()
    );

    let mut cheats = HashMap::new();

    for (x, y) in iproduct!(0..grid[0].len(), 0..grid.len()) {
        for cheat_positions in iproduct!(-20..=20, -20..=20) {
            if grid[y][x] == '#' {
                continue;
            }
            let (x_cheat, y_cheat) = (
                x as isize + cheat_positions.0,
                y as isize + cheat_positions.1,
            );
            if cheat_positions.0.abs() + cheat_positions.1.abs() > 20 {
                continue;
            }
            if x_cheat < 0 || x_cheat >= grid[0].len() as isize {
                continue;
            }
            if y_cheat < 0 || y_cheat >= grid.len() as isize {
                continue;
            }
            let (x_cheat, y_cheat) = (x_cheat as usize, y_cheat as usize);
            if grid[y_cheat][x_cheat] == '#' {
                continue;
            }
            let savings = *tiles_to_end.get(&(x_cheat, y_cheat)).unwrap() as isize
                - *tiles_to_end.get(&(x, y)).unwrap() as isize
                - cheat_positions.0.abs()
                - cheat_positions.1.abs();

            if savings > 0 {
                cheats.insert(((x, y), (x_cheat, y_cheat)), savings as usize);
            }
        }
    }

    println!(
        "Part 2: {:?}",
        cheats.values().filter(|&&v| v >= 100).count()
    );
}
