use crate::utils::read_lines;

pub fn part_1() -> u32 {
    let lines = read_lines(1);
    split_by_elfs(&lines).into_iter().max().unwrap()
}

pub fn part_2() -> u32 {
    let mut elfs = split_by_elfs(&read_lines(1));
    elfs.sort();
    elfs.reverse();
    elfs.into_iter().take(3).sum()
}

fn split_by_elfs(lines: &Vec<String>) -> Vec<u32> {
    let mut elfs = vec![];

    let mut food = 0;
    for line in lines {
        if line.is_empty() {
            elfs.push(food);
            food = 0;

            continue;
        }

        food += line.parse::<u32>().unwrap();
    }

    elfs
}
