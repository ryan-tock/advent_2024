use itertools::Itertools;
use std::collections::{HashSet, LinkedList};
use std::ops::Sub;

const INPUT: &str = include_str!("input.txt");
const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn main() {
    let robots = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| {
                    x.split("=")
                        .skip(1)
                        .next()
                        .unwrap()
                        .split(",")
                        .map(|x| x.parse::<isize>().unwrap())
                        .collect_tuple::<(isize, isize)>()
                        .unwrap()
                })
                .collect_tuple::<((isize, isize), (isize, isize))>()
                .unwrap()
        })
        .collect_vec();

    println!("Part 1: {}", quadrant_score(&iterate_n(&robots, 100)));

    let mut iterations = 0;
    loop {
        iterations += 1;
        let positions = iterate_n(&robots, iterations);
        if largest_set(&positions) > 18 {
            println!("Part 2: {}", iterations);
            break;
        }
    }
}

fn iterate_n(
    robots: &Vec<((isize, isize), (isize, isize))>,
    iterations: isize,
) -> Vec<(isize, isize)> {
    robots
        .iter()
        .map(|&(p, v)| {
            (
                (p.0 + v.0 * iterations).rem_euclid(WIDTH),
                (p.1 + v.1 * iterations).rem_euclid(HEIGHT),
            )
        })
        .collect_vec()
}

fn quadrant_score(positions: &Vec<(isize, isize)>) -> isize {
    let mut quadrants = [0isize; 4];

    for pos in positions {
        if pos.0 == WIDTH / 2 || pos.1 == HEIGHT / 2 {
            continue;
        }
        let x_quad = if pos.0 < WIDTH / 2 { 0 } else { 2 };
        let y_quad = if pos.1 < HEIGHT / 2 { 0 } else { 1 };

        quadrants[(y_quad + x_quad) as usize] += 1;
    }
    quadrants.iter().product()
}

fn print_positions(positions: &Vec<(isize, isize)>) {
    let mut grid = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
    for pos in positions {
        grid[pos.1 as usize][pos.0 as usize] = '#';
    }
    let output_string = grid.iter().map(|r| r.iter().join("")).join("\n");

    println!("{}", output_string);
}

fn largest_set(positions: &Vec<(isize, isize)>) -> isize {
    let mut unseen: HashSet<(isize, isize)> = HashSet::new();
    for i in positions {
        unseen.insert(*i);
    }
    let mut sets: Vec<HashSet<(isize, isize)>> = Vec::new();

    while !unseen.is_empty() {
        let pos = unseen.iter().next().unwrap();
        let flooded = flood(pos, positions);

        unseen = unseen.sub(&flooded);
        sets.push(flooded);
    }

    sets.iter().map(|x| x.len()).max().unwrap() as isize
}

fn flood(pos: &(isize, isize), grid: &Vec<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut in_region = HashSet::new();
    in_region.insert(*pos);
    let mut queue = LinkedList::new();
    queue.push_back(*pos);

    let mut grid_set = HashSet::new();
    for pos in grid {
        grid_set.insert(*pos);
    }

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let mut new_pos = Vec::new();

        if x > 0 {
            new_pos.push((x - 1, y));
        }
        if x < WIDTH - 1 {
            new_pos.push((x + 1, y));
        }
        if y > 0 {
            new_pos.push((x, y - 1));
        }
        if y < HEIGHT - 1 {
            new_pos.push((x, y + 1));
        }

        for pos in new_pos {
            if grid_set.contains(&pos) {
                if !in_region.contains(&pos) {
                    in_region.insert(pos);
                    queue.push_back(pos);
                }
            }
        }
    }

    in_region
}
