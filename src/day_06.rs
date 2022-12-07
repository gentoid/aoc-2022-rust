use crate::utils::read_input_to_string;

pub fn part_1() -> usize {
    let input = read_input_to_string(6);
    find_start(&input)
}

fn find_start(line: &str) -> usize {
    for index in 0..line.len() - 4 {
        if !contains_dublicates(&line[index..index+4]) {
            return index + 4;
        }
    }

    0
}

fn contains_dublicates(line: &str) -> bool {
    for (index, char) in line.char_indices() {
        let slice = &line[index + 1..];

        if slice.is_empty() {
            return false;
        }

        if slice.find(char).is_some() {
            return true;
        }
    }

    false
}
