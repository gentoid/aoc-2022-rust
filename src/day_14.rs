use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_lines;

pub fn part_1() {
    let walls = read_lines(14)
        .iter()
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

    println!("Found {} unique pairs:\n{:#?}", pairs.len(), pairs);

    // let mut coords = walls[0];

    println!("Min: {:?}", min);
    println!("Max: {:?}", max);

    for y in 0..=max.y {
        print!("{y}");
        for x in min.x..=max.x {
            if walls[0].contains(&Coord { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
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
