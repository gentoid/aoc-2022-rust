use itertools::Itertools;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let data = Data::new(read_lines(8).iter().map(parse_line).collect_vec());

    data.visibility_map()
        .iter()
        .map(|column| column.iter().filter(|cell| **cell).count())
        .sum()
}

pub fn part_2() -> usize {
    let data = Data::new(read_lines(8).iter().map(parse_line).collect_vec());

    *(data
        .scenic_scores()
        .iter()
        .map(|column| column.iter().max().unwrap())
        .max()
        .unwrap())
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

    fn scenic_scores(&self) -> Vec<Vec<usize>> {
        let x_len = self.data[0].len();
        let y_len = self.data.len();
        let mut output = vec![vec![0; x_len]; y_len];

        for y in 0..y_len {
            for x in 0..x_len {
                let coord = Coord { x, y };

                let x1 = x - self.with_x_range(&coord, x, 0).unwrap_or(0);
                let x2 = self.with_x_range(&coord, x + 1, x_len).unwrap_or(x_len - 1) - x;
                let y1 = y - self.with_y_range(&coord, y, 0).unwrap_or(0);
                let y2 = self.with_y_range(&coord, y + 1, y_len).unwrap_or(y_len - 1) - y;

                output[y][x] = x1 * x2 * y1 * y2;
            }
        }

        output
    }

    fn with_x_range(&self, coord: &Coord, from: usize, to: usize) -> Option<usize> {
        let closure = |x: &usize| self.data[coord.y][*x] >= self.data[coord.y][coord.x];

        self.find(from, to, closure)
    }

    fn with_y_range(&self, coord: &Coord, from: usize, to: usize) -> Option<usize> {
        let closure = |y: &usize| self.data[*y][coord.x] >= self.data[coord.y][coord.x];

        self.find(from, to, closure)
    }

    fn find<F>(&self, from: usize, to: usize, closure: F) -> Option<usize>
    where
        F: Fn(&usize) -> bool,
    {
        if from < to {
            (from..to).find(closure)
        } else {
            (to..from).rev().find(closure)
        }
    }
}
