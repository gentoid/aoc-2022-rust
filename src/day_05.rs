use std::str::Chars;

use itertools::{Chunk, Itertools};
use regex::Regex;

use crate::utils::{read_input_to_string, string_to_lines};

pub fn part_1() -> String {
    process(false)
}

pub fn part_2() -> String {
    process(true)
}

fn process(all_at_once: bool) -> String {
    let (stacks, instructions) = split_into_parts(read_input_to_string(5));
    let mut stacks = parse_stacks(stacks);

    for instruction in instructions.iter().map(parse_instruction) {
        stacks = move_crates(stacks, instruction, all_at_once);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&'_').to_owned())
        .collect()
}

fn split_into_parts(input: String) -> (Vec<String>, Vec<String>) {
    let parts = input.split("\n\n").collect_vec();
    assert_eq!(parts.len(), 2);

    (string_to_lines(parts[0]), string_to_lines(parts[1]))
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

fn move_crates(
    mut stacks: Vec<Vec<char>>,
    instruction: Instruction,
    all_at_once: bool,
) -> Vec<Vec<char>> {
    let mut tmp = (0..instruction.quantity)
        .map(|_| stacks[instruction.from - 1].pop().unwrap())
        .collect_vec();

    if all_at_once {
        tmp.reverse();
    }

    stacks[instruction.to - 1].extend(tmp);

    stacks
}
