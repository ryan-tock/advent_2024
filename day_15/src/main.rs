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

    for i in instructions {
        step(&mut board, &mut pos, i);
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
}

fn step(board: &mut Vec<Vec<char>>, pos: &mut (isize, isize), dir: (isize, isize)) {
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
