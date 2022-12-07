use std::str::Chars;

use itertools::{Chunk, Itertools};

use crate::utils::read_lines;

pub fn part_1() -> u32 {
    let (stacks, instructions) = split_into_parts(read_lines(5));
    let stacks = parse_stacks(stacks);

    println!("{:?}", stacks);

    0
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
