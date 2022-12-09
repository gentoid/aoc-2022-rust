use std::ops::Range;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let data = Data::new(read_lines(8).iter().map(parse_line).collect_vec());

    data.visibility_map()
        .iter()
        .map(|column| column.iter().filter(|cell| **cell).count())
        .sum()
}

fn parse_line(line: &String) -> Vec<u32> {
    line.chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect_vec()
}

#[derive(Debug)]
struct Data {
    data: Vec<Vec<u32>>,
}

impl Data {
    fn new(data: Vec<Vec<u32>>) -> Self {
        Self { data }
    }

    fn visibility_map(&self) -> Vec<Vec<bool>> {
        let x_len = self.data.len();
        let y_len = self.data[0].len();
        let mut output = vec![vec![true; y_len]; x_len];

        for x in 0..x_len - 1 {
            for y in 0..y_len - 1 {
                let tree = self.data[x][y];

                output[x][y] = self.is_visible_horizontally(tree, y, 0..x)
                    || self.is_visible_horizontally(tree, y, x + 1..x_len)
                    || self.is_visible_vertically(tree, x, 0..y)
                    || self.is_visible_vertically(tree, x, y + 1..y_len);
            }
        }

        output
    }

    fn is_visible_horizontally(&self, tree: u32, y: usize, range: Range<usize>) -> bool {
        for check_tree in range.map(|x| self.data[x][y]) {
            if check_tree >= tree {
                return false;
            }
        }

        true
    }

    fn is_visible_vertically(&self, tree: u32, x: usize, range: Range<usize>) -> bool {
        for check_tree in range.map(|y| self.data[x][y]) {
            if check_tree >= tree {
                return false;
            }
        }

        true
    }
}
