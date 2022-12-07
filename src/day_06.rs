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
        if !contains_duplicates(&line[index..index + uniques]) {
            return index + uniques;
        }
    }

    0
}

fn contains_duplicates(line: &str) -> bool {
    line.char_indices()
        .find(|(index, char)| line[index + 1..].find(*char).is_some())
        .is_some()
}
