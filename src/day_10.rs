use crate::utils::read_lines;

pub fn part_1() -> i32 {
    let mut state = 1;
    let mut cycles = 0;
    let mut next_cycles_value = 20;

    let mut output = 0;

    for instruction in read_lines(10).iter().map(parse_line) {
        let (add_cycles, new_state) = run_instruction(state, &instruction);
        let new_cycles = cycles + add_cycles;

        if new_cycles >= next_cycles_value {
            let strength = state * next_cycles_value as i32;
            output += strength;
            next_cycles_value += 40;
        }

        state = new_state;
        cycles = new_cycles;
    }

    output
}

enum Instruction {
    Noop,
    Add(i32),
}

fn parse_line(line: &String) -> Instruction {
    if line == "noop" {
        return Instruction::Noop;
    }

    if line.starts_with("addx ") {
        return Instruction::Add(line[5..].parse::<i32>().unwrap());
    }

    unreachable!();
}

fn run_instruction(state: i32, instruction: &Instruction) -> (u32, i32) {
    match instruction {
        Instruction::Noop => (1, state),
        Instruction::Add(value) => (2, state + value),
    }
}
