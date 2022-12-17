use std::{fmt::Debug, ops::Range, os::windows::process};

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

    // (min_x..=max_x)
    //     .map(|x| {
    //         let coord = Coord { x, y };
    //         scanners
    //             .iter()
    //             .find(|scanner| scanner.can_be_scanned(&coord))
    //             .is_some()
    //     })
    //     .filter(|found| *found)
    //     .count()
    //     - beacons_on_line

    let mut ranges = scanners
        .iter()
        .map(|sensor| sensor.range_for_y(&y))
        .filter(|range| !range.is_empty())
        .collect_vec();

    println!("Ranges:\n{:?}", ranges);

    // let mut merged_ranges = vec![];

    let mut iterations = 0;
    loop {
        let (merged, merged_at_least_once) = merge_ranges(&ranges);
        ranges = merged;
        iterations += 1;

        if !merged_at_least_once {
            break;
        }

        println!("[{iterations}] {:?}", ranges);
    }

    println!("Iterations: {iterations}");
    println!("Merged ranges:\n{:?}", ranges);

    0
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

    fn range_for_y(&self, y: &i32) -> Range<i32> {
        let y_diff = self.coord.y.abs_diff(*y) as i32;

        if y_diff > self.radius as i32 {
            return 0..0; // Empty range
        }

        let x_diff = self.radius as i32 - y_diff;
        (self.coord.x - x_diff)..(self.coord.x + x_diff + 1)
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

fn range_union<N>(r1: &Range<N>, r2: &Range<N>) -> Option<Range<N>>
where
    N: Copy + Ord + PartialOrd,
{
    if r1.start <= r2.start && r1.end > r2.start {
        return Some(r1.start..r1.end.max(r2.end));
    }

    if r2.start <= r1.start && r2.end > r1.start {
        return Some(r2.start..r1.end.max(r2.end));
    }

    None
}

fn merge_ranges<N>(ranges: &[Range<N>]) -> (Vec<Range<N>>, bool)
where
    N: Copy + Debug + Ord + PartialOrd,
{
    let mut merged_at_least_once = false;
    let mut merged_ranges = vec![];
    let mut processed_ranges = vec![];

    println!("  Got {} ranges", ranges.len());

    for (index, r1) in ranges.iter().enumerate() {
        let inner_start = index + 1;
        let mut merged = false;

        if inner_start >= ranges.len() {
            if !processed_ranges.contains(&index) {
                println!("  [{index}] Adding the last item in the range");
                merged_ranges.push(r1.clone());
            }
            break;
        }

        if processed_ranges.contains(&index) {
            continue;
        }

        for (inner_index, r2) in ranges[inner_start..].iter().enumerate() {
            let inner_index = inner_index + inner_start;

            if processed_ranges.contains(&inner_index) {
                continue;
            }

            println!("  Compare {:?} and {:?}", r1, r2);
            if let Some(range) = range_union(r1, r2) {
                println!("    [{index}], [{inner_index}] Merged to: {:?}", range);
                merged_ranges.push(range);
                processed_ranges.push(index);
                processed_ranges.push(inner_index);
                merged = true;
                merged_at_least_once = true;
                break;
            }
        }

        if !merged {
            println!("    [{index}] Wasn't nerged, so push too");
            merged_ranges.push(r1.clone());
        } else {
            println!("    [{index}] Was already merged");
        }
    }

    (merged_ranges, merged_at_least_once)
}
