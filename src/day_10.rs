use itertools::Itertools;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let mut position = 1;
    let mut cycles = 0;
    let mut next_cycles_value = 20;

    let mut output = 0;

    for instruction in read_lines(10).iter().map(parse_line) {
        let (add_cycles, new_position) = run_instruction(position, &instruction);
        let new_cycles = cycles + add_cycles;

        if new_cycles >= next_cycles_value {
            let strength = position * next_cycles_value;
            output += strength;
            next_cycles_value += 40;
        }

        position = new_position;
        cycles = new_cycles;
    }

    output
}

pub fn part_2() {
    let mut beam_position = 1;
    let mut cycles = 0;
    let mut cycles_offset = 0;
    let beam_width = 3;
    let screen_width = 40;

    let mut pixels = vec![];

    for instruction in read_lines(10).iter().map(parse_line) {
        let (add_cycles, new_position) = run_instruction(beam_position, &instruction);

        for _ in 0..add_cycles {
            let cycles_position = cycles + 1 - cycles_offset;
            let pixel =
                cycles_position >= beam_position && cycles_position < (beam_position + beam_width);
            // println!("[{cycles}] => {pixel} : {cycles_position} | {beam_position}");
            pixels.push(pixel);

            if cycles_position == screen_width {
                cycles_offset += screen_width;
            }

            cycles += 1;
        }

        beam_position = new_position;
    }

    visualize(&pixels, screen_width)
}

enum Instruction {
    Noop,
    Add(i32),
}

fn parse_line(line: &String) -> Instruction {
    match &line[0..4] {
        "noop" => Instruction::Noop,
        "addx" =>Instruction::Add(line[5..].parse::<i32>().unwrap()),
        _ => unreachable!(),
    }
}

fn run_instruction(position: usize, instruction: &Instruction) -> (usize, usize) {
    match instruction {
        Instruction::Noop => (1, position),
        Instruction::Add(value) => (2, (position as i32 + value) as usize),
    }
}

fn visualize(pixels: &[bool], width: usize) {
    for line in &pixels.iter().chunks(width) {
        let output_line: String = line.map(|pixel| if *pixel { '#' } else { ' ' }).collect();
        println!("{output_line}");
    }
}
