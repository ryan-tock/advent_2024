use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| if x == '^' { '.' } else { x })
                .collect_vec()
        })
        .collect_vec();

    let guard_index = INPUT
        .replace("\n", "")
        .chars()
        .position(|c| c == '^')
        .unwrap();

    let start_pos = (guard_index % grid[0].len(), guard_index / grid[0].len());
    let start_pos = (start_pos.0 as i32, start_pos.1 as i32);
    let start_dir = (0i32, -1i32);

    let mut pos = start_pos;
    let mut dir = start_dir;

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    loop {
        if visited.contains(&(dir, pos)) {
            break;
        }
        visited.insert((dir, pos));

        let mut newx = pos.0 + dir.0;
        let mut newy = pos.1 + dir.1;
        if newx < 0 || newy < 0 || newx >= grid[0].len() as i32 || newy >= grid.len() as i32 {
            break;
        }
        while grid[newy as usize][newx as usize] != '.' {
            dir = (-dir.1, dir.0);
            newx = pos.0 + dir.0;
            newy = pos.1 + dir.1;
        }

        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    let visited = visited.iter().map(|x| x.1).collect::<HashSet<_>>();

    println!("Part 1: {}", visited.len());

    let mut loop_count = 0;

    for new_obstacle_pos in visited {
        if new_obstacle_pos == start_pos {
            continue;
        }
        let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        let mut pos = start_pos;
        let mut dir = start_dir;

        let is_loop = loop {
            if visited.contains(&(dir, pos)) {
                break true;
            }
            visited.insert((dir, pos));

            let mut newx = pos.0 + dir.0;
            let mut newy = pos.1 + dir.1;
            if newx < 0 || newy < 0 || newx >= grid[0].len() as i32 || newy >= grid.len() as i32 {
                break false;
            }
            while grid[newy as usize][newx as usize] == '#' || (newx, newy) == new_obstacle_pos {
                dir = (-dir.1, dir.0);
                newx = pos.0 + dir.0;
                newy = pos.1 + dir.1;
            }

            pos.0 += dir.0;
            pos.1 += dir.1;
        };
        if is_loop {
            loop_count += 1;
        }
    }

    println!("Part 2: {}", loop_count);
}
