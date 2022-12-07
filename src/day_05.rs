use std::str::Chars;

use itertools::{Chunk, Itertools};
use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> String {
    let (stacks, instructions) = split_into_parts(read_lines(5));
    let mut stacks = parse_stacks(stacks);

    for instruction in instructions.iter().map(parse_instruction) {
        stacks = move_crates(stacks, instruction);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&'_').to_owned())
        .collect()
}

fn split_into_parts(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut stacks = vec![];
    let mut instructions = vec![];

    let mut stacks_data = true;

    for line in lines.into_iter() {
        if line.is_empty() {
            stacks_data = false;
            continue;
        }

        if stacks_data {
            stacks.push(line);
        } else {
            instructions.push(line);
        }
    }

    (stacks, instructions)
}

fn parse_stacks(mut lines: Vec<String>) -> Vec<Vec<char>> {
    let num_of_stacks = number_of_stacks(lines.pop().unwrap());
    let mut stacks = vec![vec![]; num_of_stacks];

    lines.reverse();

    for line in lines {
        for (index, chunk) in line.chars().chunks(4).into_iter().enumerate() {
            if let Some(char) = stack_value(chunk) {
                stacks[index].push(char);
            }
        }
    }

    stacks
}

fn number_of_stacks(line: String) -> usize {
    line.chars().chunks(4).into_iter().collect_vec().len()
}

fn stack_value(mut value: Chunk<Chars>) -> Option<char> {
    value
        .nth(1)
        .and_then(|value| if value == ' ' { None } else { Some(value) })
}

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_instruction(line: &String) -> Instruction {
    let template = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let captures = template.captures(line).unwrap();

    Instruction {
        quantity: captures[1].parse::<usize>().unwrap(),
        from: captures[2].parse::<usize>().unwrap(),
        to: captures[3].parse::<usize>().unwrap(),
    }
}

fn move_crates(mut stacks: Vec<Vec<char>>, instruction: Instruction) -> Vec<Vec<char>> {
    let mut from = stacks[instruction.from - 1].clone();
    let to = &mut stacks[instruction.to - 1];

    for _ in 0..instruction.quantity {
        to.push(from.pop().unwrap());
    }

    stacks[instruction.from - 1] = from;

    stacks
}
