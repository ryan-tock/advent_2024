use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const NUM_FALL: usize = 1024;

fn main() {
    let positions = INPUT
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .collect_vec();

    let grid_size = positions.iter().map(|x| x.0 + 1).max().unwrap();

    let mut grid = vec![vec!['.'; grid_size]; grid_size];
    for &(x, y) in positions[0..NUM_FALL].iter() {
        grid[y][x] = '#';
    }

    // println!("{}", grid.iter().map(|x| x.iter().join("")).join("\n"));

    let mut visited = HashSet::new();
    let mut current_positions = vec![(0usize, 0usize)];
    let mut step_count = 0;

    loop {
        let mut next_step = Vec::new();
        for (x, y) in current_positions {
            let (x, y) = (x as isize, y as isize);
            let mut new_positions = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
            new_positions.retain(|&(x, y)| {
                x >= 0 && x < grid_size as isize && y >= 0 && y < grid_size as isize
            });

            let mut new_positions = new_positions
                .iter()
                .map(|&(x, y)| (x as usize, y as usize))
                .collect_vec();

            new_positions.retain(|&(x, y)| grid[y][x] != '#');
            new_positions.retain(|&(x, y)| !visited.contains(&(x, y)));

            for &element in new_positions.iter() {
                next_step.push(element);
                visited.insert(element);
            }
        }

        current_positions = next_step;
        step_count += 1;
        if visited.contains(&(grid_size - 1, grid_size - 1)) {
            break;
        }
    }

    println!("Part 1: {}", step_count);

    let mut num_falls = NUM_FALL + 1;
    loop {
        let mut grid = vec![vec!['.'; grid_size]; grid_size];
        for &(x, y) in positions[0..num_falls].iter() {
            grid[y][x] = '#';
        }

        let mut visited = HashSet::new();
        let mut current_positions = vec![(0usize, 0usize)];

        let works = loop {
            let mut next_step = Vec::new();
            for (x, y) in current_positions {
                let (x, y) = (x as isize, y as isize);
                let mut new_positions = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
                new_positions.retain(|&(x, y)| {
                    x >= 0 && x < grid_size as isize && y >= 0 && y < grid_size as isize
                });

                let mut new_positions = new_positions
                    .iter()
                    .map(|&(x, y)| (x as usize, y as usize))
                    .collect_vec();

                new_positions.retain(|&(x, y)| grid[y][x] != '#');
                new_positions.retain(|&(x, y)| !visited.contains(&(x, y)));

                for &element in new_positions.iter() {
                    next_step.push(element);
                    visited.insert(element);
                }
            }
            current_positions = next_step;
            if visited.contains(&(grid_size - 1, grid_size - 1)) {
                break true;
            }
            if current_positions.is_empty() {
                break false;
            }
        };

        if !works {
            println!(
                "Part 2: \"{},{}\"",
                positions[num_falls - 1].0,
                positions[num_falls - 1].1
            );
            break;
        }

        num_falls += 1;
    }
}
