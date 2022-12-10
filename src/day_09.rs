use std::collections::HashSet;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let mut rope = Rope::new();

    for movement in read_lines(9).iter().map(parse_line) {
        rope.process(movement);
    }

    rope.visited.len()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn diff(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Rope {
    head: Coord,
    tail: Coord,
    visited: HashSet<Coord>,
}

impl Rope {
    fn new() -> Self {
        let init_coord = Coord { x: 0, y: 0 };
        let mut visited = HashSet::new();
        visited.insert(init_coord.clone());

        Self {
            head: init_coord.clone(),
            tail: init_coord.clone(),
            visited,
        }
    }

    fn process(&mut self, (direction, amount): (Direction, u32)) {
        use Direction::*;

        if amount == 0 {
            return;
        }

        match direction {
            Up => self.head.y += 1,
            Down => self.head.y -= 1,
            Left => self.head.x -= 1,
            Right => self.head.x += 1,
        }

        let diff = self.head.diff(&self.tail);

        if diff.x.abs() > 1 {
            self.tail.y = self.head.y;

            match direction {
                Left => self.tail.x = self.head.x + 1,
                Right => self.tail.x = self.head.x - 1,
                _ => unreachable!(),
            }
        } else if diff.y.abs() > 1 {
            self.tail.x = self.head.x;

            match direction {
                Up => self.tail.y = self.head.y - 1,
                Down => self.tail.y = self.head.y + 1,
                _ => unreachable!(),
            }
        }

        self.visited.insert(self.tail.clone());

        self.process((direction, amount - 1));
    }
}

fn parse_line(line: &String) -> (Direction, u32) {
    use Direction::*;

    let amount = line[2..].parse::<u32>().unwrap();

    match &line[0..1] {
        "U" => (Up, amount),
        "D" => (Down, amount),
        "L" => (Left, amount),
        "R" => (Right, amount),
        substr => unreachable!("Got '{substr}'"),
    }
}
