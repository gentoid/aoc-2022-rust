use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

pub fn part_1() -> usize {
    calculate(2, false).visited.len()
}

pub fn part_2() -> usize {
    calculate(10, false).visited.len()
}

fn calculate(length: usize, visualize_output: bool) -> Rope {
    let mut rope = Rope::new(length);

    for (direction, amount) in read_lines(9).iter().map(parse_line) {
        rope.process(&direction, amount, visualize_output);
    }

    rope
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

    fn process(&mut self, direction: &Direction, amount: u32, visualize_output: bool) {
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

        if visualize_output {
            visualize(&self, &format!("====== {amount} {:?} ======", direction));
        }

        self.process_rest(1, visualize_output);

        let tail_index = self.rope.len() - 1;
        self.visited.insert(self.rope[tail_index].clone());

        self.process(&direction, amount - 1, visualize_output);
    }

    fn process_rest(&mut self, index: usize, visualize_output: bool) {
        if index == 0 || index >= self.rope.len() {
            return;
        }

        let prev_index = index - 1;
        let diff = self.rope[prev_index].diff(&self.rope[index]);

        let mut changed = false;

        if diff.x.abs() > 1 && diff.y.abs() > 1 {
            self.rope[index].x += diff.x.signum();
            self.rope[index].y += diff.y.signum();
            changed = true;
        } else if diff.x.abs() > 1 {
            self.rope[index].x += diff.x.signum();
            self.rope[index].y = self.rope[prev_index].y;
            changed = true;
        } else if diff.y.abs() > 1 {
            self.rope[index].x = self.rope[prev_index].x;
            self.rope[index].y += diff.y.signum();
            changed = true;
        }

        if changed {
            self.process_rest(index + 1, visualize_output);

            if visualize_output {
                visualize(&self, &format!("  >> rest: {index}"));
            }
        }
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

fn visualize(rope: &Rope, title: &str) {
    let mut min = Coord { x: 0, y: 0 };
    let mut max = Coord { x: 0, y: 0 };
    let mut rope_parts = HashMap::new();

    for (index, coord) in rope.rope.iter().enumerate() {
        get_min_max(coord, &mut min, &mut max);

        if let None = rope_parts.get_mut(&(coord.x, coord.y)) {
            rope_parts.insert((coord.x, coord.y), index);
        }
    }

    for coord in &rope.visited {
        get_min_max(coord, &mut min, &mut max);
    }

    println!("{title}");
    println!("");

    for y in (min.y..=max.y).rev() {
        let mut line = String::new();

        for x in min.x..=max.x {
            match rope_parts.get(&(x, y)) {
                None => {
                    if x == 0 && y == 0 {
                        line.push('s')
                    } else {
                        line.push('.')
                    }
                }
                Some(index) => line.push(char::from_digit(*index as u32, 10).unwrap()),
            }
        }

        line.extend("    ".chars());

        for x in min.x..=max.x {
            match &rope.visited.get(&Coord { x, y }) {
                None => {
                    if x == 0 && y == 0 {
                        line.push('s')
                    } else {
                        line.push('.')
                    }
                }
                Some(_) => line.push('#'),
            }
        }

        println!("{line}");
        println!("");
    }
}

fn get_min_max(coord: &Coord, min: &mut Coord, max: &mut Coord) {
    if coord.x < min.x {
        min.x = coord.x;
    }
    if coord.x > max.x {
        max.x = coord.x;
    }
    if coord.y < min.y {
        min.y = coord.y;
    }
    if coord.y > max.y {
        max.y = coord.y;
    }
}
