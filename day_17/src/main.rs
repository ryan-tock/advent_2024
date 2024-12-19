use itertools::Itertools;
use std::ops::{BitXor, BitXorAssign};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let sections = INPUT.split("\n\n").collect_vec();
    let mut registers = sections[0]
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap())
        .collect_tuple::<(usize, usize, usize)>()
        .unwrap();

    let program = sections[1]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let mut instruction_pointer = 0;

    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let op_code = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match op_code {
            0 => registers.0 /= 2usize.pow(get_combo(&registers, operand) as u32),
            1 => registers.1.bitxor_assign(operand),
            2 => registers.1 = get_combo(&registers, operand) % 8,
            3 => {
                if registers.0 != 0 {
                    instruction_pointer = operand;
                } else {
                    instruction_pointer += 2;
                }
            }
            4 => registers.1.bitxor_assign(registers.2),
            5 => output.push(get_combo(&registers, operand) % 8),
            6 => registers.1 = registers.0 / 2usize.pow(get_combo(&registers, operand) as u32),
            7 => registers.2 = registers.0 / 2usize.pow(get_combo(&registers, operand) as u32),
            _ => unreachable!(),
        }

        if op_code != 3 {
            instruction_pointer += 2;
        }
    }

    println!(
        "Part 1: {:?}",
        output.iter().map(|x| x.to_string()).join(",")
    );

    let program = program.iter().map(|x| x.bitxor(6)).collect_vec();

    let mut possible = vec![0usize];

    for op_code in program.iter().rev() {
        let mut next_possible = Vec::new();
        for entry in possible {
            for guess in 0usize..8 {
                let a_register = entry * 8 + guess;
                let mut b_register = guess.bitxor(1);
                let c_register = (a_register) / 2usize.pow(b_register as u32);
                b_register.bitxor_assign(c_register);
                if b_register % 8 == *op_code {
                    next_possible.push(a_register);
                }
            }
        }
        possible = next_possible;
    }

    println!("Part 2: {:?}", possible.iter().min().unwrap());
}

fn get_combo(registers: &(usize, usize, usize), x: usize) -> usize {
    if x <= 3 {
        return x;
    }
    if x <= 6 {
        match x - 4 {
            0 => return registers.0,
            1 => return registers.1,
            2 => return registers.2,
            _ => unreachable!(),
        }
    }
    panic!("Invalid program! input number is {x}");
}
