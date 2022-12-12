use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let map = Map::new();

    let mut paths = vec![Path::new(&map)];
    let mut next_paths = vec![];
    let mut visited = HashSet::new();

    for _ in 0..10000 {
        for path in paths {
            let tmp = path
                .find_more_paths(&map)
                .into_iter()
                .filter(|path| !visited.iter().contains(&path.current.0))
                .collect_vec();

            tmp.iter().for_each(|path| {
                visited.insert(path.current.0.clone());
            });

            next_paths.extend(tmp);
        }

        if let Some(found) = next_paths.iter().find(|path| path.current.0 == map.end) {
            return found.path.len() - 1;
        }

        paths = next_paths;
        next_paths = vec![];
    }

    0
}

struct Map {
    map: Vec<Vec<u32>>,
    max: Coord,
    start: Coord,
    end: Coord,
}

impl Map {
    fn new() -> Self {
        let mut map = vec![];
        let mut start = Coord { x: 0, y: 0 };
        let mut end = Coord { x: 0, y: 0 };
        let mut max = Coord { x: 0, y: 0 };
        read_lines(12)
            .iter()
            .map(|line| line.chars().collect_vec())
            .enumerate()
            .for_each(|(y, chars)| {
                max.y = max.y.max(y);

                let mut line = vec![];
                for (x, char) in chars.iter().enumerate() {
                    max.x = max.x.max(x);

                    match char {
                        'a'..='z' => line.push(*char as u32),
                        'S' => {
                            line.push('a' as u32);
                            start = Coord { x, y };
                        }
                        'E' => {
                            line.push('z' as u32);
                            end = Coord { x, y };
                        }
                        _ => unreachable!(),
                    }
                }

                map.push(line);
            });

        Map {
            map,
            start,
            end,
            max,
        }
    }

    fn at(&self, coord: &Coord) -> u32 {
        self.map[coord.y][coord.x]
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug)]
struct Path {
    visited: HashSet<Coord>,
    path: Vec<Coord>,
    current: (Coord, u32),
}

impl Path {
    fn new(map: &Map) -> Self {
        let mut visited = HashSet::new();
        visited.insert(map.start.clone());

        Self {
            visited,
            path: vec![map.start.clone()],
            current: (map.start.clone(), map.at(&map.start)),
        }
    }

    fn find_more_paths(self, map: &Map) -> Vec<Self> {
        self.current
            .0
            .neighbors(&map.max)
            .into_iter()
            .filter(|coord| !self.visited.iter().contains(&coord))
            .filter(|coord| map.at(coord) <= self.current.1 + 1)
            .map(|coord| {
                let mut path = self.clone();
                path.visited.insert(coord.clone());
                path.path.push(coord.clone());
                path.current = (coord.clone(), map.at(&coord));

                path
            })
            .collect_vec()
    }
}
