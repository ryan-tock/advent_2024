use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let mut trailheads = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                trailheads.insert((x, y));
            }
        }
    }

    let mut total_score = 0;

    for trailhead in trailheads.iter() {
        let trailhead = *trailhead;
        let mut reached_positions = HashSet::new();
        reached_positions.insert(trailhead);

        for _i in 0..9 {
            let mut new_reached_positions = HashSet::new();
            for position in reached_positions.iter() {
                for new_pos in get_valid_neighbors(&grid, position) {
                    new_reached_positions.insert(new_pos);
                }
            }

            reached_positions = new_reached_positions;
        }

        total_score += reached_positions.len()
    }

    println!("Part 1: {}", total_score);

    let mut total_score = 0;

    for trailhead in trailheads.iter() {
        let trailhead = *trailhead;

        let mut reached_positions = Vec::new();
        reached_positions.push(trailhead);

        for _i in 0..9 {
            let mut new_reached_positions = Vec::new();
            for position in reached_positions.iter() {
                for new_pos in get_valid_neighbors(&grid, position) {
                    new_reached_positions.push(new_pos);
                }
            }

            reached_positions = new_reached_positions;
        }

        total_score += reached_positions.len()
    }

    println!("Part 2: {}", total_score);
}

fn get_valid_neighbors(grid: &Vec<Vec<usize>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let (x, y) = *pos;
    let target = grid[y][x] + 1;

    if x > 0 && grid[y][x - 1] == target {
        neighbors.push((x - 1, y));
    }
    if x < grid.len() - 1 && grid[y][x + 1] == target {
        neighbors.push((x + 1, y));
    }
    if y > 0 && grid[y - 1][x] == target {
        neighbors.push((x, y - 1));
    }
    if y < grid[0].len() - 1 && grid[y + 1][x] == target {
        neighbors.push((x, y + 1));
    }

    neighbors
}
