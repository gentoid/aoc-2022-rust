use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    read_lines(4)
        .iter()
        .map(parse_line)
        .filter(|ranges| fully_covered(&ranges))
        .count()
}

struct Range {
    from: u32,
    to: u32,
}

impl Range {
    pub fn covered_by(&self, other: &Range) -> bool {
        other.from <= self.from && other.to >= self.to
    }
}

fn parse_line(line: &String) -> (Range, Range) {
    let temlate = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let captures = temlate.captures(line).unwrap();

    (
        Range {
            from: captures[1].parse::<u32>().unwrap(),
            to: captures[2].parse::<u32>().unwrap(),
        },
        Range {
            from: captures[3].parse::<u32>().unwrap(),
            to: captures[4].parse::<u32>().unwrap(),
        },
    )
}

fn fully_covered(ranges: &(Range, Range)) -> bool {
    ranges.0.covered_by(&ranges.1) || ranges.1.covered_by(&ranges.0)
}
