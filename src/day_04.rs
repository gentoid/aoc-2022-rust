use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    read_lines(4)
        .iter()
        .map(parse_line)
        .filter(|ranges| fully_covered(&ranges))
        .count()
}

pub fn part_2() -> usize {
    read_lines(4)
        .iter()
        .map(parse_line)
        .filter(|ranges| overlapped(&ranges))
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

    pub fn overlap_by(&self, other: &Range) -> bool {
        Self::in_range(self.from, other) || Self::in_range(self.to, other)
    }

    fn in_range(num: u32, range: &Range) -> bool {
        num >= range.from && num <= range.to
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

fn overlapped(ranges: &(Range, Range)) -> bool {
    ranges.0.overlap_by(&ranges.1) || ranges.1.overlap_by(&ranges.0)
}
