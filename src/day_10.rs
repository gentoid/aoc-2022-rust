use itertools::Itertools;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let mut screen = Screen::new(40, Some(20));

    let mut output = 0;

    for instruction in read_lines(10).iter().map(parse_line) {
        screen.run_instruction(&instruction);

        if let Some(strength) = screen.signal_strength {
            output += strength;
        }
    }

    output
}

pub fn part_2() {
    let mut screen = Screen::new(40, None);

    for instruction in read_lines(10).iter().map(parse_line) {
        screen.run_instruction(&instruction);
    }

    screen.visualize();
}

struct Screen {
    beam_position: usize,
    beam_width: usize,
    cycles: usize,
    width: usize,
    first_line_width: usize,
    current_line: usize,
    pixels: Vec<bool>,
    signal_strength: Option<usize>,
}

impl Screen {
    fn new(width: usize, first_line_width: Option<usize>) -> Self {
        Self {
            beam_position: 1,
            beam_width: 3,
            cycles: 0,
            width,
            first_line_width: first_line_width.unwrap_or(width),
            current_line: 1,
            pixels: vec![],
            signal_strength: None,
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        let (add_cycles, position_change) = self.get_change(instruction);

        self.signal_strength = None;

        for _ in 0..add_cycles {
            let mut offset = 0;

            if self.current_line > 1 {
                offset = self.first_line_width + self.width * (self.current_line - 2);
            }

            let cycles_position = self.cycles + 1 - offset;
            let pixel = cycles_position >= self.beam_position
                && cycles_position < (self.beam_position + self.beam_width);

            self.pixels.push(pixel);

            let current_line_width = if self.current_line == 1 {
                self.first_line_width
            } else {
                self.width
            };

            if cycles_position == current_line_width {
                self.current_line += 1;
                self.signal_strength = Some((offset + current_line_width) * self.beam_position);
            }

            self.cycles += 1;
        }

        self.beam_position = (self.beam_position as isize + position_change) as usize;
    }

    fn get_change(&self, instruction: &Instruction) -> (usize, isize) {
        match instruction {
            Instruction::Noop => (1, 0),
            Instruction::Add(value) => (2, *value),
        }
    }

    fn visualize(&self) {
        for line in &self.pixels.iter().chunks(self.width) {
            let output_line: String = line.map(|pixel| if *pixel { '#' } else { ' ' }).collect();
            println!("{output_line}");
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(isize),
}

fn parse_line(line: &String) -> Instruction {
    match &line[0..4] {
        "noop" => Instruction::Noop,
        "addx" => Instruction::Add(line[5..].parse::<isize>().unwrap()),
        _ => unreachable!(),
    }
}
