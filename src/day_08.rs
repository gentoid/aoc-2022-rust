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
    x_len: usize,
    y_len: usize,
}

impl Data {
    fn new(data: Vec<Vec<u32>>) -> Self {
        let x_len = data[0].len();
        let y_len = data.len();
        Self { data, x_len, y_len }
    }

    fn visibility_map(&self) -> Vec<Vec<bool>> {
        self.iterate(true, |_, x1, x2, y1, y2| {
            x1.is_none() || x2.is_none() || y1.is_none() || y2.is_none()
        })
    }

    fn scenic_scores(&self) -> Vec<Vec<usize>> {
        self.iterate(0, |coord, x1, x2, y1, y2| {
            let x1 = coord.x - x1.unwrap_or(0);
            let x2 = x2.unwrap_or(self.x_len - 1) - coord.x;
            let y1 = coord.y - y1.unwrap_or(0);
            let y2 = y2.unwrap_or(self.y_len - 1) - coord.y;

            x1 * x2 * y1 * y2
        })
    }

    fn iterate<F, T>(&self, init: T, closure: F) -> Vec<Vec<T>>
    where
        T: Default + Clone,
        F: Fn(&Coord, Option<usize>, Option<usize>, Option<usize>, Option<usize>) -> T,
    {
        let mut output = vec![vec![init; self.x_len]; self.y_len];

        for y in 0..self.y_len {
            for x in 0..self.x_len {
                let coord = Coord { x, y };

                let x1 = self.with_x_range(&coord, x, 0);
                let x2 = self.with_x_range(&coord, x + 1, self.x_len);
                let y1 = self.with_y_range(&coord, y, 0);
                let y2 = self.with_y_range(&coord, y + 1, self.y_len);

                output[y][x] = closure(&coord, x1, x2, y1, y2);
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
