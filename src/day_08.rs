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

struct Coord {
    x: usize,
    y: usize,
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
                let coord = Coord { x, y };

                output[x][y] = self.with_x_range(&coord, 0, x).is_none()
                    || self.with_x_range(&coord, x + 1, x_len).is_none()
                    || self.with_y_range(&coord, 0, y).is_none()
                    || self.with_y_range(&coord, y + 1, y_len).is_none();
            }
        }

        output
    }

    fn with_x_range(&self, coord: &Coord, from: usize, to: usize) -> Option<usize> {
        let tree = self.data[coord.x][coord.y];
        (from..to).find(|x| self.data[*x][coord.y] >= tree)
    }

    fn with_y_range(&self, coord: &Coord, from: usize, to: usize) -> Option<usize> {
        let tree = self.data[coord.x][coord.y];
        (from..to).find(|y| self.data[coord.x][*y] >= tree)
    }
}
