use itertools::Itertools;
use regex::Regex;

pub fn part_1() -> u32 {
    0
}

fn parse_monkey(input: &str) -> Monkey {
    let (if_true, if_false) = parse_throws(input);

    Monkey {
        items: parse_items(input),
        operation: parse_operation(input),
        test: parse_test(input),
        if_true,
        if_false,
    }
}

fn parse_items(input: &str) -> Vec<u32> {
    let template = Regex::new(r"Starting items: ((?:\d+, )*\d+)").unwrap();
    let captures = template.captures(input).unwrap();
    captures[1]
        .split(", ")
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec()
}

fn parse_operation(input: &str) -> Box<dyn Fn(u32) -> u32> {
    let template = Regex::new(r"Operation: new = old ([+*]) (\d+)").unwrap();
    let captures = template.captures(input).unwrap();

    let value = captures[2].parse::<u32>().unwrap();

    match &captures[1] {
        "+" => Box::new(move |i| i + value),
        "*" => Box::new(move |i| i * value),
        _ => unreachable!(),
    }
}

fn parse_test(input: &str) -> Box<dyn Fn(u32) -> bool> {
    let template = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let captures = template.captures(input).unwrap();

    let value = captures[1].parse::<u32>().unwrap();

    Box::new(move |i| i % value == 0)
}

fn parse_throws(input: &str) -> (usize, usize) {
    let template =
        Regex::new(r"If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();
    let captures = template.captures(input).unwrap();

    let if_true = captures[1].parse::<usize>().unwrap();
    let if_false = captures[2].parse::<usize>().unwrap();

    (if_true, if_false)
}

struct Monkey {
    items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> bool>,
    if_true: usize,
    if_false: usize,
}
