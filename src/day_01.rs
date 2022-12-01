use crate::utils::read_lines;

pub fn part_1() -> u32 {
    let lines = read_lines(1);
    split_by_elfs(&lines)
        .iter()
        .map(|food| food.iter().sum())
        .max()
        .unwrap()
}

fn split_by_elfs(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut elfs = vec![];

    let mut elf = vec![];
    for line in lines {
        if line.is_empty() {
            elfs.push(elf);
            elf = vec![];

            continue;
        }

        elf.push(line.parse::<u32>().unwrap())
    }

    elfs
}
