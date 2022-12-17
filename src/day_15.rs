use itertools::Itertools;
use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let (scanners, beacons): (Vec<Sensor>, Vec<Coord>) =
        read_lines(15).iter().map(|line| parse_line(&line)).unzip();

    let y = 2000000;

    let min_x = scanners
        .iter()
        .map(|scanner| scanner.coord.x - scanner.radius as i32)
        .min()
        .unwrap();

    let max_x = scanners
        .iter()
        .map(|scanner| scanner.coord.x + scanner.radius as i32)
        .max()
        .unwrap();

    let beacons_on_line = beacons
        .into_iter()
        .unique()
        .filter(|beacon| beacon.y == y)
        .count();

    (min_x..=max_x)
        .map(|x| {
            let coord = Coord { x, y };
            scanners
                .iter()
                .find(|scanner| scanner.can_be_scanned(&coord))
                .is_some()
        })
        .filter(|found| *found)
        .count()
        - beacons_on_line
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

struct Sensor {
    coord: Coord,
    radius: usize,
}

impl Sensor {
    fn can_be_scanned(&self, coord: &Coord) -> bool {
        let diff_x = self.coord.x.abs_diff(coord.x);
        let diff_y = self.coord.y.abs_diff(coord.y);

        (diff_x + diff_y) as usize <= self.radius
    }
}

fn parse_line(line: &str) -> (Sensor, Coord) {
    let template =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let captures = template.captures(line).unwrap();

    let sensor = Coord {
        x: captures[1].parse::<i32>().unwrap(),
        y: captures[2].parse::<i32>().unwrap(),
    };

    let beacon = Coord {
        x: captures[3].parse::<i32>().unwrap(),
        y: captures[4].parse::<i32>().unwrap(),
    };

    let diff_x = sensor.x.abs_diff(beacon.x);
    let diff_y = sensor.y.abs_diff(beacon.y);

    (
        Sensor {
            coord: sensor,
            radius: (diff_x + diff_y) as usize,
        },
        beacon,
    )
}
