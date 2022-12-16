use core::fmt;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_lines;

type WallCorners = Vec<Coord>;
type Cells = HashMap<Coord, CellType>;
type Pairs = HashSet<(Coord, Coord)>;

pub fn part_1() -> usize {
    solve(&read_lines(14))
}

pub fn solve(input: &[String]) -> usize {
    let walls = input
        .iter()
        .unique()
        .map(|line| parse_line(line))
        .collect_vec();

    let pairs = extract_pairs(&walls);

    println!("Found {} unique pairs", pairs.len());

    let mut cave: Cave = prepare_cave(pairs);

    println!("Min: {:?}", cave.min);
    println!("Max: {:?}", cave.max);

    cave.new_particle();

    loop {
        cave.tick();

        if cave.fullfilled {
            println!("No more particles can be held");
            break;
        }

        if let None = cave.current_particle {
            cave.new_particle();
        }
    }

    for y in 0..=cave.max.y {
        print!("{y}");
        for x in cave.min.x..=cave.max.x {
            let coord = Coord { x, y };
            if coord == cave.start_coord {
                print!("S");
            } else if let Some(cell_type) = cave.cells.get(&coord) {
                print!("{cell_type}");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    cave.cells
        .iter()
        .filter(|cell| *cell.1 == CellType::Sand)
        .count()
}

#[derive(PartialEq)]
enum CellType {
    Wall,
    Sand,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_as = match self {
            CellType::Sand => 'o',
            CellType::Wall => '#',
        };

        write!(f, "{display_as}")
    }
}

enum BottomType {
    Floor,
    Void,
}

struct Bottom {
    bottom_type: BottomType,
    add_levels: usize,
}

struct Cave {
    cells: Cells,
    start_coord: Coord,
    current_particle: Option<Coord>,
    bottom: Bottom,
    fullfilled: bool,
    min: Coord,
    max: Coord,
}

impl Cave {
    fn new(cells: Cells) -> Self {
        let wall_cells = cells
            .iter()
            .filter(|(_, cell_type)| **cell_type == CellType::Wall)
            .map(|(coord, _)| coord.clone())
            .collect_vec();

        Self {
            cells,
            start_coord: Coord { x: 500, y: 0 },
            current_particle: None,
            bottom: Bottom {
                bottom_type: BottomType::Void,
                add_levels: 0,
            },
            fullfilled: false,
            min: wall_cells.iter().fold(Coord::max(), Coord::min_of),
            max: wall_cells.iter().fold(Coord::min(), Coord::max_of),
        }
    }

    fn new_particle(&mut self) {
        if self.cells.contains_key(&self.start_coord) {
            self.fullfilled = true;
            return;
        }

        self.current_particle = Some(self.start_coord.clone());
        self.cells.insert(self.start_coord.clone(), CellType::Sand);
    }

    fn tick(&mut self) {
        let current_particle = self
            .current_particle
            .clone()
            .unwrap_or(self.start_coord.clone());

        let next_coord = current_particle.down();
        let moved = self.move_particle(&current_particle, next_coord);

        if moved {
            return;
        }

        let next_coord = current_particle.down_left();
        let moved = self.move_particle(&current_particle, next_coord);

        if moved {
            return;
        }

        let next_coord = current_particle.down_right();
        let moved = self.move_particle(&current_particle, next_coord);

        if moved {
            return;
        }

        self.current_particle = None;
    }

    fn move_particle(&mut self, from: &Coord, to: Coord) -> bool {
        if from.y >= (self.max.y + self.bottom.add_levels) {
            self.fullfilled = true;
            self.current_particle = None;
            self.cells.remove(&from);
            return true;
        }

        if !self.cells.contains_key(&to) {
            let value = self.cells.remove(&from).unwrap();
            self.cells.insert(to.clone(), value);
            self.current_particle = Some(to);
            return true;
        }

        false
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn min() -> Self {
        Self { x: 0, y: 0 }
    }

    fn max() -> Self {
        Self {
            x: usize::MAX,
            y: usize::MAX,
        }
    }

    fn min_of(one: Self, other: &Self) -> Self {
        Self {
            x: one.x.min(other.x),
            y: one.y.min(other.y),
        }
    }

    fn max_of(one: Self, other: &Self) -> Self {
        Self {
            x: one.x.max(other.x),
            y: one.y.max(other.y),
        }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn parse_line(line: &str) -> Vec<Coord> {
    line.split(" -> ")
        .map(|part| parse_cord(part))
        .collect_vec()
}

fn parse_cord(input: &str) -> Coord {
    let parts = input.split(",").collect_vec();

    Coord {
        x: parts[0].parse::<usize>().unwrap(),
        y: parts[1].parse::<usize>().unwrap(),
    }
}

fn min_max(one: &usize, two: &usize) -> (usize, usize) {
    (*one.min(two), *one.max(two))
}

fn extract_pairs(walls: &[WallCorners]) -> Pairs {
    let mut pairs = HashSet::new();

    for wall in walls {
        for pair in wall.windows(2) {
            let one = pair[0].clone();
            let two = pair[1].clone();

            pairs.insert(if one > two { (two, one) } else { (one, two) });
        }
    }

    pairs
}

fn prepare_cave(pairs: Pairs) -> Cave {
    let mut cells = HashMap::new();

    for (from, to) in pairs {
        if from.x != to.x {
            let (min, max) = min_max(&from.x, &to.x);
            let y = from.y;

            for x in min..=max {
                cells.insert(Coord { x, y }, CellType::Wall);
            }
        } else if from.x != to.y {
            let (min, max) = min_max(&from.y, &to.y);
            let x = from.x;

            for y in min..=max {
                cells.insert(Coord { x, y }, CellType::Wall);
            }
        }
    }

    Cave::new(cells)
}
