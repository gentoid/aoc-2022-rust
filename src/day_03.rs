use crate::utils::read_lines;

pub fn part_1() -> u32 {
    read_lines(3)
        .iter()
        .map(|line| priority(&find_common_item(&split_string(line))))
        .sum()
}

pub fn part_2() -> u32 {
    let lines = read_lines(3);

    lines
        .chunks(3)
        .map(|chunk| priority(&find_common_item(chunk)))
        .sum()
}

fn split_string(line: &String) -> Vec<String> {
    let length = line.len();
    vec![line[..length / 2].to_owned(), line[length / 2..].to_owned()]
}

fn find_common_item(chunk: &[String]) -> Vec<char> {
    let chars = match chunk.len() {
        0 | 1 => vec![],
        2 => chunk[1].chars().collect(),
        _ => find_common_item(&chunk[1..]),
    };

    chars
        .into_iter()
        .filter(|char| chunk[0].chars().find(|inner_char| inner_char == char).is_some())
        .collect()
}

fn priority(char: &Vec<char>) -> u32 {
    if let Some(char) = char.get(0) {
        match char {
            'a'..='z' => *char as u32 - 'a' as u32 + 1,
            'A'..='Z' => *char as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        }
    } else {
        0
    }
}
