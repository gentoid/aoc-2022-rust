use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> u32 {
    read_lines(2).iter().map(|line| score(parse_line(&line))).sum()
}

pub fn part_2() -> u32 {
    read_lines(2).iter().map(|line| score_updated(parse_line(&line))).sum()
}

#[derive(Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: &String) -> (Shape, Shape) {
    let temlate = Regex::new(r"^(\w) (\w)$").unwrap();
    let captures = temlate.captures(line).unwrap();

    let opponent_shape = match &captures[1] {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => unreachable!(),
    };

    let my_shape = match &captures[2] {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => unreachable!(),
    };

    (opponent_shape, my_shape)
}

const LOST: u32 = 0;
const DRAW: u32 = 3;
const WON: u32 = 6;

fn score((opponent_shape, my_shape): (Shape, Shape)) -> u32 {
    use Shape::*;

    let score_1 = match my_shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let score_2 = match (my_shape, opponent_shape) {
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => LOST,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => WON,
        (_, _) => DRAW,
    };

    score_1 + score_2
}

enum Strategy {
    ToLose,
    ToDraw,
    ToWin,
}

fn score_updated((opponent_shape, my_shape): (Shape, Shape)) -> u32 {
    use Shape::*;
    use Strategy::*;

    let strategy = match my_shape {
        Rock => ToLose,
        Paper => ToDraw,
        Scissors => ToWin,
    };

    let my_shape = match (&opponent_shape, strategy) {
        (Rock, ToLose) | (Paper, ToWin) => Scissors,
        (Rock, ToWin) | (Scissors, ToLose) => Paper,
        (Paper, ToLose) | (Scissors, ToWin) => Rock,
        (shape, ToDraw) => shape.clone(),
    };

    score((opponent_shape, my_shape))
}
