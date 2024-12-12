use itertools::Itertools;
use std::collections::{HashSet, LinkedList};
use std::ops::Sub;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut unseen = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            unseen.insert((x, y));
        }
    }

    let mut regions = Vec::new();

    while !unseen.is_empty() {
        let region = flood(unseen.iter().next().unwrap(), &grid);
        unseen = unseen.sub(&region);
        regions.push(region);
    }

    let mut total_score = 0;

    for region in regions.iter() {
        let mut perim = 0;
        for &(x, y) in region.iter() {
            if x == 0 || x == grid[y].len() - 1 {
                perim += 1;
            }
            if y == 0 || y == grid.len() - 1 {
                perim += 1;
            }
            if x > 0 && !region.contains(&(x - 1, y)) {
                perim += 1;
            }
            if x < grid.len() - 1 && !region.contains(&(x + 1, y)) {
                perim += 1;
            }
            if y > 0 && !region.contains(&(x, y - 1)) {
                perim += 1;
            }
            if y < grid.len() - 1 && !region.contains(&(x, y + 1)) {
                perim += 1;
            }
        }
        let area = region.len();
        // let point = *region.iter().next().unwrap();
        // println!(
        //     "For region '{}': area {} and perim {}",
        //     grid[point.1][point.0], area, perim
        // );
        total_score += perim * area;
    }

    println!("Part 1: {}", total_score);

    let mut total_score = 0;

    for region in regions.iter() {
        let mut sides = 0;
        for &(x, y) in region.iter() {
            let mut edges = [0usize; 4];

            if x == 0 || !region.contains(&(x - 1, y)) {
                edges[0] = 1;
            }
            if x == grid.len() - 1 || !region.contains(&(x + 1, y)) {
                edges[1] = 1;
            }
            if y == 0 || !region.contains(&(x, y - 1)) {
                edges[2] = 1;
            }
            if y == grid.len() - 1 || !region.contains(&(x, y + 1)) {
                edges[3] = 1;
            }
            let num_edges = edges.iter().sum::<usize>();
            if num_edges > 0 {
                sides += num_edges - 1;
            }
            if num_edges == 4 {
                sides += 1;
            }
            if num_edges == 2 {
                if (edges[0] == 1 && edges[1] == 1) || (edges[2] == 1 && edges[3] == 1) {
                    sides -= 1;
                }
            }

            if x > 0 && y > 0 {
                if region.contains(&(x, y - 1))
                    && region.contains(&(x - 1, y))
                    && !region.contains(&(x - 1, y - 1))
                {
                    sides += 1;
                }
            }
            if x > 0 && y < grid.len() - 1 {
                if region.contains(&(x, y + 1))
                    && region.contains(&(x - 1, y))
                    && !region.contains(&(x - 1, y + 1))
                {
                    sides += 1;
                }
            }
            if x < grid.len() - 1 && y > 0 {
                if region.contains(&(x, y - 1))
                    && region.contains(&(x + 1, y))
                    && !region.contains(&(x + 1, y - 1))
                {
                    sides += 1;
                }
            }
            if x < grid.len() - 1 && y < grid.len() - 1 {
                if region.contains(&(x, y + 1))
                    && region.contains(&(x + 1, y))
                    && !region.contains(&(x + 1, y + 1))
                {
                    sides += 1;
                }
            }
        }
        let area = region.len();
        // let point = *region.iter().next().unwrap();
        // println!(
        //     "For region '{}': area {} and sides {}",
        //     grid[point.1][point.0], area, sides
        // );
        total_score += sides * area;
    }

    println!("Part 2: {}", total_score);
}

fn flood(pos: &(usize, usize), grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut in_region = HashSet::new();
    in_region.insert(*pos);
    let mut queue = LinkedList::new();
    queue.push_back(*pos);

    let target_char = grid[pos.1][pos.0];

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let mut new_pos = Vec::new();

        if x > 0 {
            new_pos.push((x - 1, y));
        }
        if x < grid[0].len() - 1 {
            new_pos.push((x + 1, y));
        }
        if y > 0 {
            new_pos.push((x, y - 1));
        }
        if y < grid.len() - 1 {
            new_pos.push((x, y + 1));
        }

        for pos in new_pos {
            if grid[pos.1][pos.0] == target_char {
                if !in_region.contains(&pos) {
                    in_region.insert(pos);
                    queue.push_back(pos);
                }
            }
        }
    }

    in_region
}
