use std::collections::HashSet;

use itertools::Itertools;

fn parse_line(line: &String) -> Vec<u32> {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbors(&self, max: &Coord) -> Vec<Self> {
        let mut output = vec![];

        if self.x > 0 {
            output.push(self.with_x(self.x - 1));
        }

        if self.x < max.x {
            output.push(self.with_x(self.x + 1));
        }

        if self.y > 0 {
            output.push(self.with_y(self.y - 1));
        }

        if self.y < max.y {
            output.push(self.with_y(self.y + 1));
        }

        output
    }

    fn with_x(&self, x: usize) -> Self {
        let mut coord = self.clone();
        coord.x = x;
        coord
    }

    fn with_y(&self, y: usize) -> Self {
        let mut coord = self.clone();
        coord.y = y;
        coord
    }
}

#[derive(Clone)]
struct Path {
    visited: HashSet<Coord>,
    path: Vec<Coord>,
    current: (Coord, u32),
    max: Coord,
}

impl Path {
    fn find_more_paths(self, map: &Vec<Vec<u32>>) -> Vec<Self> {
        self.current.0
            .neighbors(&self.max)
            .into_iter()
            .filter(|coord| !self.visited.iter().contains(&coord))
            .filter(|coord| map[coord.y][coord.x] <= self.current.1 + 1)
            .map(|coord| {
                let mut path = self.clone();
                path.visited.insert(coord.clone());
                path.path.push(coord.clone());
                path.current = (coord.clone(), map[coord.y][coord.x]);

                path
            })
            .collect_vec()
    }
}
