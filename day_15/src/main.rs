use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let sections = INPUT.split("\n\n").collect_vec();
    let mut board = sections[0]
        .split("\n")
        .map(|x| x.chars().collect_vec())
        .collect_vec();
    let instructions = sections[1]
        .split("\n")
        .join("")
        .chars()
        .map(|x| {
            if x == '<' {
                return (-1, 0);
            };
            if x == '>' {
                return (1, 0);
            };
            if x == '^' {
                return (0, -1);
            };
            (0, 1)
        })
        .collect_vec();

    let mut pos: (isize, isize) = (0, 0);
    for line_index in 0..board.len() {
        for char_index in 0..board[0].len() {
            if board[line_index][char_index] == '@' {
                pos = (char_index as isize, line_index as isize);
                board[line_index][char_index] = '.';
            }
        }
    }

    for i in instructions.iter() {
        step_1(&mut board, &mut pos, *i);
    }

    let gps_score = board
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, symbol)| if *symbol == 'O' { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("Part 1: {}", gps_score);

    let mut pos: (isize, isize) = (0, 0);

    let mut board: Vec<Vec<char>> = vec![vec!['.'; board[0].len() * 2]; board.len()];
    for (y, line) in sections[0].split("\n").enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == '@' {
                pos = ((x * 2) as isize, y as isize);
            }
            if symbol == 'O' {
                board[y][x * 2] = '[';
                board[y][x * 2 + 1] = ']';
            }
            if symbol == '#' {
                board[y][x * 2] = '#';
                board[y][x * 2 + 1] = '#';
            }
        }
    }
    // print_board(&board, pos);

    for i in instructions.iter() {
        if can_step(&board, &pos, *i) {
            step_2(&mut board, &pos, *i);
            pos.0 += i.0;
            pos.1 += i.1;
        }
        // print_board(&board, pos);
    }

    let gps_score = board
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, symbol)| if *symbol == '[' { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("Part 2: {}", gps_score);
}

fn step_1(board: &mut Vec<Vec<char>>, pos: &mut (isize, isize), dir: (isize, isize)) {
    let mut new_pos = (pos.0 + dir.0, pos.1 + dir.1);

    while board[new_pos.1 as usize][new_pos.0 as usize] == 'O' {
        new_pos.0 += dir.0;
        new_pos.1 += dir.1;
    }
    if board[new_pos.1 as usize][new_pos.0 as usize] == '#' {
        return;
    }

    board[new_pos.1 as usize][new_pos.0 as usize] = 'O';
    pos.0 += dir.0;
    pos.1 += dir.1;
    board[pos.1 as usize][pos.0 as usize] = '.';
}

fn can_step(board: &Vec<Vec<char>>, pos: &(isize, isize), dir: (isize, isize)) -> bool {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if board[new_pos.1 as usize][new_pos.0 as usize] == '#' {
        return false;
    }
    if board[new_pos.1 as usize][new_pos.0 as usize] == '.' {
        return true;
    }
    if dir.1 == 0 {
        return can_step(board, &new_pos, dir);
    }
    let additional_pos = if board[new_pos.1 as usize][new_pos.0 as usize] == '[' {
        (new_pos.0 + 1, new_pos.1)
    } else {
        (new_pos.0 - 1, new_pos.1)
    };

    can_step(board, &new_pos, dir) && can_step(board, &additional_pos, dir)
}
fn step_2(board: &mut Vec<Vec<char>>, pos: &(isize, isize), dir: (isize, isize)) {
    let symbol = board[pos.1 as usize][pos.0 as usize];
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if board[new_pos.1 as usize][new_pos.0 as usize] == '.' {
        board[pos.1 as usize][pos.0 as usize] = '.';
        board[new_pos.1 as usize][new_pos.0 as usize] = symbol;
        return;
    }

    if dir.1 == 0 {
        step_2(board, &new_pos, dir);
    } else {
        let additional_pos = if board[new_pos.1 as usize][new_pos.0 as usize] == '[' {
            (new_pos.0 + 1, new_pos.1)
        } else {
            (new_pos.0 - 1, new_pos.1)
        };

        step_2(board, &new_pos, dir);
        step_2(board, &additional_pos, dir);
    }

    board[pos.1 as usize][pos.0 as usize] = '.';
    board[new_pos.1 as usize][new_pos.0 as usize] = symbol;
}

fn print_board(board: &Vec<Vec<char>>, pos: (isize, isize)) {
    let mut new_board = board.clone();
    new_board[pos.1 as usize][pos.0 as usize] = '@';
    println!(
        "{}\n",
        new_board.iter().map(|x| x.iter().join("")).join("\n")
    );
}
