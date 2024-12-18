use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub struct State {
    pub last_state: Option<Rc<State>>,
    pub pos: (usize, usize),
    pub dir: (isize, isize),
    pub score: usize,
}

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Axis {
    NS,
    EW,
}

fn main() {
    let mut grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start_pos = find_char(&grid, 'S');
    let end_pos = find_char(&grid, 'E');
    grid[start_pos.1][start_pos.0] = '.';
    grid[end_pos.1][end_pos.0] = '.';

    let mut queue = BinaryHeap::from([
        State {
            last_state: None,
            pos: start_pos,
            dir: (1, 0),
            score: 0,
        },
        State {
            last_state: None,
            pos: start_pos,
            dir: (0, -1),
            score: 1000,
        },
    ]);

    let mut seen_vertices = HashMap::new();
    seen_vertices.insert((start_pos, to_axis((1, 0))), 0);
    seen_vertices.insert((start_pos, to_axis((0, -1))), 1000);

    let mut best_paths: Vec<State> = Vec::new();
    loop {
        let curr_state = queue.pop().unwrap();
        if curr_state.pos == end_pos {
            if best_paths.len() == 0 || curr_state.score == best_paths[0].score {
                best_paths.push(curr_state);
            } else {
                break;
            }
        } else {
            let new_states = next_states(&grid, &mut seen_vertices, Rc::new(curr_state));
            for state in new_states {
                queue.push(state);
            }
        }
    }
    let best_score = best_paths[0].score;
    println! {"Part 1: {}", best_score};

    let mut path_tiles = HashSet::new();
    path_tiles.insert(end_pos);

    loop {
        let curr_state = best_paths.pop();
        match curr_state {
            Some(_) => {}
            None => {
                break;
            }
        }
        let curr_state = curr_state.unwrap();
        let mut retrace_state = &curr_state.last_state;
        loop {
            match retrace_state {
                Some(state) => {
                    path_tiles.insert(state.pos);
                    retrace_state = &state.last_state;
                }
                None => {
                    break;
                }
            }
        }
    }

    println! {"Part 2: {}", path_tiles.len()};

    // println!("seen vertices: {:?}", seen_vertices);
    // print_path(&grid, queue.peek().unwrap());
}

fn print_path(grid: &Vec<Vec<char>>, state: &State) {
    let mut set = HashSet::new();
    let curr_state = state;
    set.insert(curr_state.pos);
    let mut curr_state = &curr_state.last_state;
    loop {
        match curr_state {
            Some(state) => {
                set.insert(state.pos);
                curr_state = &state.last_state;
            }
            None => break,
        }
    }

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if set.contains(&(x, y)) {
                print!(".");
            } else {
                if *c == '.' {
                    print!(" ");
                } else {
                    print!("{c}");
                }
            }
        }
        println!();
    }
}

fn next_states(
    grid: &Vec<Vec<char>>,
    seen_vertices: &mut HashMap<((usize, usize), Axis), usize>,
    state: Rc<State>,
) -> Vec<State> {
    let (x, y) = state.pos;
    let (mut x, mut y) = (x as isize, y as isize);

    x += state.dir.0;
    y += state.dir.1;

    if grid[y as usize][x as usize] == '#' {
        return Vec::new();
    }

    let turn_1_dir = (state.dir.1, state.dir.0);
    let turn_1_pos = ((x + turn_1_dir.0) as usize, (y + turn_1_dir.1) as usize);
    let turn_2_dir = (-state.dir.1, -state.dir.0);
    let turn_2_pos = ((x + turn_2_dir.0) as usize, (y + turn_2_dir.1) as usize);
    let forward_pos = (x as usize, y as usize);

    let forward_state = State {
        last_state: Some(state.clone()),
        pos: (x as usize, y as usize),
        score: state.score + 1,
        ..*state
    };

    if grid[turn_1_pos.1][turn_1_pos.0] == '#' && grid[turn_2_pos.1][turn_2_pos.0] == '#' {
        return vec![forward_state];
    }
    let mut new_states = Vec::new();
    let straight_score = seen_vertices
        .entry((forward_pos, to_axis(state.dir)))
        .or_insert(usize::MAX);

    if state.score + 1 <= *straight_score {
        *straight_score = state.score + 1;
        new_states.push(forward_state);
    }

    let turn_score = seen_vertices
        .entry((turn_1_pos, to_axis(turn_1_dir)))
        .or_insert(usize::MAX);

    let mut turned = false;

    if grid[turn_1_pos.1][turn_1_pos.0] != '#' && state.score + 1001 <= *turn_score {
        turned = true;
        new_states.push(State {
            last_state: Some(state.clone()),
            pos: (x as usize, y as usize),
            dir: turn_1_dir,
            score: state.score + 1001,
        });
    }

    if grid[turn_2_pos.1][turn_2_pos.0] != '#' && state.score + 1001 <= *turn_score {
        turned = true;
        new_states.push(State {
            last_state: Some(state.clone()),
            pos: (x as usize, y as usize),
            dir: turn_2_dir,
            score: state.score + 1001,
        });
    }

    if turned {
        *turn_score = state.score + 1001;
    }

    new_states
}

fn to_axis(direction: (isize, isize)) -> Axis {
    if direction.0 != 0 {
        Axis::EW
    } else {
        Axis::NS
    }
}

fn find_char(grid: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == target {
                return (x, y);
            }
        }
    }
    unreachable!();
}
