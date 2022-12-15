use core::fmt;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_lines;

type Cave = HashMap<Coord, CellType>;

pub fn part_1() {
    let walls = read_lines(14)
        .iter()
        .unique()
        .map(|line| parse_line(line))
        .collect_vec();

    let min = by_condition(&walls, Coord::max(), Coord::min_of);
    let max = by_condition(&walls, Coord::min(), Coord::max_of);

    let mut pairs = HashSet::new();

    for wall in &walls {
        for pair in wall.windows(2) {
            let one = pair[0].clone();
            let two = pair[1].clone();

            pairs.insert(if one > two { (two, one) } else { (one, two) });
        }
    }

    println!("Found {} unique pairs", pairs.len());

    println!("Min: {:?}", min);
    println!("Max: {:?}", max);

    let mut cave: Cave = HashMap::new();

    for (from, to) in pairs {
        if from.x != to.x {
            let (min, max) = min_max(&from.x, &to.x);
            let y = from.y;

            for x in min..=max {
                cave.insert(Coord { x, y }, CellType::Wall);
            }
        } else if from.x != to.y {
            let (min, max) = min_max(&from.y, &to.y);

            let x = from.x;
            for y in min..=max {
                cave.insert(Coord { x, y }, CellType::Wall);
            }
        }
    }

    let mut particle = Coord { x: 500, y: 0 };
    cave.insert(particle.clone(), CellType::Sand);

    let mut counter = 0;
    let mut particles = 0;
    let floor = max.y + 1;

    loop {
        let (updated_cave, next_particle, add_more) = tick(cave, &floor, &particle);

        cave = updated_cave;
        if !add_more {
            println!("No more particles can be held");
            break;
        }

        counter += 1;

        if let Some(next_coord) = next_particle {
            particle = next_coord;
            continue;
        } else {
            particles += 1;
            particle = Coord { x: 500, y: 0 };
            cave.insert(particle.clone(), CellType::Sand);
        }
    }

    println!("Counter end: {counter}");
    println!("Added {particles} particles");

    for y in 0..=max.y {
        print!("{y}");
        for x in min.x..=max.x {
            if x == 500 && y == 0 {
                print!("S");
            } else if let Some(cell_type) = cave.get(&Coord { x, y }) {
                print!("{cell_type}");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

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

    fn min_of(one: &Coord, other: &Self) -> Self {
        Self {
            x: one.x.min(other.x),
            y: one.y.min(other.y),
        }
    }

    fn max_of(one: &Coord, other: &Self) -> Self {
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

fn by_condition(
    coords: &Vec<Vec<Coord>>,
    init: Coord,
    select: fn(&Coord, &Coord) -> Coord,
) -> Coord {
    coords.iter().fold(init, |result, wall| {
        let in_wall = wall
            .iter()
            .fold(result.clone(), |inner, coord| select(&inner, coord));
        select(&result, &in_wall)
    })
}

fn min_max(one: &usize, two: &usize) -> (usize, usize) {
    (*one.min(two), *one.max(two))
}

fn tick(cave: Cave, floor: &usize, particle: &Coord) -> (Cave, Option<Coord>, bool) {
    let next_coord = particle.down();

    if next_coord.y >= *floor {
        return (cave, None, false);
    }

    let (cave, moved) = move_particle(cave, particle, &next_coord);

    if moved {
        return (cave, Some(next_coord), true);
    }

    let next_coord = particle.down_left();
    let (cave, moved) = move_particle(cave, particle, &next_coord);

    if moved {
        return (cave, Some(next_coord), true);
    }

    let next_coord = particle.down_right();
    let (cave, moved) = move_particle(cave, particle, &next_coord);

    if moved {
        return (cave, Some(next_coord), true);
    }

    (cave, None, true)
}

fn move_particle(mut cave: Cave, from: &Coord, to: &Coord) -> (Cave, bool) {
    if !cave.contains_key(&to) {
        let value = cave.remove(&from).unwrap();
        cave.insert(to.clone(), value);
        return (cave, true);
    }

    (cave, false)
}
