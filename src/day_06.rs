use crate::utils::read_input_to_string;

pub fn part_1() -> usize {
    let input = read_input_to_string(6);
    find_start(&input, 4)
}

pub fn part_2() -> usize {
    let input = read_input_to_string(6);
    find_start(&input, 14)
}

fn find_start(line: &str, uniques: usize) -> usize {
    for index in 0..line.len() - uniques {
        if !contains_dublicates(&line[index..index + uniques]) {
            return index + uniques;
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
