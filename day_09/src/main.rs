use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let numbers = INPUT.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    let disk_space = numbers.iter().sum::<u32>() as usize;

    let mut is_file = true;

    let mut disk: Vec<i32> = Vec::with_capacity(disk_space);
    let mut id = 0;

    let mut disk_map = Vec::new();
    let mut open_spaces: Vec<BinaryHeap<Reverse<usize>>> = Vec::new();
    for _i in 0..=9 {
        open_spaces.push(BinaryHeap::new());
    }

    for i in 0..numbers.len() {
        if is_file {
            disk_map.push((numbers[i] as usize, disk.len()));
            for _j in 0..numbers[i] as usize {
                disk.push(id);
            }
            id += 1;
        } else {
            open_spaces[numbers[i] as usize].push(Reverse(disk.len()));
            for _j in 0..numbers[i] as usize {
                disk.push(-1);
            }
        }

        is_file = !is_file;
    }

    let mut left_pointer = 0;
    let mut right_pointer = disk_space - 1;

    let mut new_disk: Vec<i32> = Vec::new();

    while left_pointer <= right_pointer {
        if disk[right_pointer] == -1 {
            right_pointer -= 1;
        }
        if disk[left_pointer] >= 0 {
            new_disk.push(disk[left_pointer]);
            left_pointer += 1;
        }
        if disk[left_pointer] < 0 && disk[right_pointer] >= 0 {
            new_disk.push(disk[right_pointer]);
            left_pointer += 1;
            right_pointer -= 1;
        }
    }

    let checksum = new_disk
        .iter()
        .enumerate()
        .map(|(i, &d)| i * d as usize)
        .sum::<usize>();
    println!("Part 1: {}", checksum);

    for i in (0..disk_map.len()).rev() {
        let (len, index) = disk_map[i];
        let mut best_gap: (usize, usize) = (usize::MAX, usize::MAX);
        for gap_len in len..=9 {
            let new_pos = open_spaces[gap_len]
                .peek()
                .unwrap_or(&Reverse(disk.len() + 1))
                .0;
            if new_pos < best_gap.0 {
                best_gap = (new_pos, gap_len);
            }
        }
        let (new_pos, gap_len) = best_gap;
        if new_pos < index {
            disk_map[i] = (len, new_pos);
            let new_open_len = gap_len - len;
            open_spaces[gap_len].pop();
            if new_open_len > 0 {
                open_spaces[new_open_len].push(Reverse(new_pos + len));
            }
        }
    }

    let triangles = (0..=8).map(|x| triangle(x)).collect_vec();

    let checksum = disk_map
        .iter()
        .enumerate()
        .map(|(i, &(len, index))| i * (len * index + triangles[len - 1]))
        .sum::<usize>();

    println!("Part 2: {}", checksum);
    // 13170001574942 is high
}

fn triangle(x: usize) -> usize {
    if x <= 1 {
        x
    } else {
        x + triangle(x - 1)
    }
}
