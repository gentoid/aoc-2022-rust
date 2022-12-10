use std::collections::HashSet;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let mut rope = Rope::new(2);

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
    rope: Vec<Coord>,
    visited: HashSet<Coord>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let init_coord = Coord { x: 0, y: 0 };

        let mut rope = vec![];

        for _ in 0..length {
            rope.push(init_coord.clone());
        }

        let mut visited = HashSet::new();
        visited.insert(init_coord.clone());

        Self { rope, visited }
    }

    fn process(&mut self, (direction, amount): (Direction, u32)) {
        use Direction::*;

        if amount == 0 {
            return;
        }

        match direction {
            Up => self.rope[0].y += 1,
            Down => self.rope[0].y -= 1,
            Left => self.rope[0].x -= 1,
            Right => self.rope[0].x += 1,
        }

        let diff = self.rope[0].diff(&self.rope[1]);

        if diff.x > 1 {
            self.rope[1].y = self.rope[0].y;
            self.rope[1].x = self.rope[0].x - 1;
        } else if diff.x < -1 {
            self.rope[1].y = self.rope[0].y;
            self.rope[1].x = self.rope[0].x + 1;
        } else if diff.y > 1 {
            self.rope[1].x = self.rope[0].x;
            self.rope[1].y = self.rope[0].y - 1;
        } else if diff.y < -1 {
            self.rope[1].x = self.rope[0].x;
            self.rope[1].y = self.rope[0].y + 1;
        }

        self.visited.insert(self.rope[1].clone());

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
