use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut letter_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    for line_index in 0..grid.len() {
        for char_index in 0..grid[line_index].len() {
            let char = grid[line_index][char_index];
            if char != '.' {
                letter_map
                    .entry(char)
                    .or_insert(HashSet::new())
                    .insert((line_index, char_index));
            }
        }
    }

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let mut antipodes: HashSet<(isize, isize)> = HashSet::new();

    for char in letter_map.keys() {
        let positions = letter_map.get(char).unwrap();
        let cartesian = iproduct!(positions.iter(), positions.iter()).collect_vec();

        for (point1, point2) in cartesian {
            if point1 == point2 {
                continue;
            }

            let point1 = (point1.0 as isize, point1.1 as isize);
            let point2 = (point2.0 as isize, point2.1 as isize);

            let new1 = (2 * point1.0 - point2.0, 2 * point1.1 - point2.1);
            let new2 = (2 * point2.0 - point1.0, 2 * point2.1 - point1.1);

            if 0 <= new1.0 && new1.0 < width && 0 <= new1.1 && new1.1 < height {
                antipodes.insert(new1);
            }
            if 0 <= new2.0 && new2.0 < width && 0 <= new2.1 && new2.1 < height {
                antipodes.insert(new2);
            }
        }
    }

    println!("Part 1: {:?}", antipodes.len());

    let mut antipodes: HashSet<(isize, isize)> = HashSet::new();

    for char in letter_map.keys() {
        let positions = letter_map.get(char).unwrap();
        let cartesian = iproduct!(positions.iter(), positions.iter()).collect_vec();

        for (point1, point2) in cartesian {
            if point1 == point2 {
                continue;
            }

            let mut point1 = (point1.0 as isize, point1.1 as isize);
            let mut point2 = (point2.0 as isize, point2.1 as isize);

            let diff1 = (point1.0 - point2.0, point1.1 - point2.1);
            let diff2 = (point2.0 - point1.0, point2.1 - point1.1);

            while (0 <= point1.0 && point1.0 < width && 0 <= point1.1 && point1.1 < height)
                || (0 <= point2.0 && point2.0 < width && 0 <= point2.1 && point2.1 < height)
            {
                if 0 <= point1.0 && point1.0 < width && 0 <= point1.1 && point1.1 < height {
                    antipodes.insert(point1);
                }
                if 0 <= point2.0 && point2.0 < width && 0 <= point2.1 && point2.1 < height {
                    antipodes.insert(point2);
                }

                point1.0 += diff1.0;
                point1.1 += diff1.1;
                point2.0 += diff2.0;
                point2.1 += diff2.1;
            }
        }
    }

    println!("Part 2: {:?}", antipodes.len());
}
