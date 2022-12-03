use crate::utils::read_lines;

pub fn part_1() -> u32 {
    read_lines(3)
        .iter()
        .map(|line| priority(&find_common_item(&parse_line(line))))
        .sum()
}

fn parse_line(line: &String) -> (String, String) {
    let length = line.len();
    (line[..length / 2].to_owned(), line[length / 2..].to_owned())
}

fn find_common_item(parts: &(String, String)) -> Option<char> {
    for char_1 in parts.0.chars() {
        for char_2 in parts.1.chars() {
            if char_1 == char_2 {
                return Some(char_1);
            }
        }
    }

    None
}

fn priority(char: &Option<char>) -> u32 {
    if let Some(char) = char {
        match char {
            'a'..='z' => *char as u32 - 'a' as u32 + 1,
            'A'..='Z' => *char as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        }
    } else {
        0
    }
}
