use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::utils::read_input_to_string;

pub fn part_1() -> u32 {
    let mut monkeys = read_input_to_string(11)
        .split("\n\n")
        .map(|input| parse_monkey(input))
        .collect_vec();

    let mut all_updates: HashMap<usize, Vec<u32>> = HashMap::new();

    for (index, monkey) in monkeys.iter_mut().enumerate() {
        println!("Monkey init {index}: {:?}", monkey.items);

        if let Some(items) = all_updates.get_mut(&index) {
            monkey.add(&items);
            *items = vec![];
        };

        println!("Monkey w/up {index}: {:?}", monkey.items);

        for (monkey, item) in monkey.turn() {
            all_updates.entry(monkey).or_default().push(item);
        }
        println!("Monkey aftr {index}: {:?}\n", monkey.items);
        println!("Updates: {:?}\n\n", all_updates);
    }

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
    let template = Regex::new(r"Operation: new = old ([+*]) ((:?\d+)|(:?\w+))").unwrap();
    let captures = template.captures(input).unwrap();

    if &captures[2] == "old" {
        return match &captures[1] {
            "+" => Box::new(move |i| i + i),
            "*" => Box::new(move |i| i * i),
            _ => unreachable!(),
        };
    }

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

impl Monkey {
    fn add(&mut self, items: &[u32]) {
        self.items.extend(items);
    }

    fn turn(&mut self) -> Vec<(usize, u32)> {
        let output = self
            .items
            .iter()
            .map(|item| self.inspect(*item))
            .collect_vec();
        self.items = vec![];
        output
    }

    fn inspect(&self, item: u32) -> (usize, u32) {
        let worry_level = (self.operation)(item) / 3;

        if (self.test)(worry_level) {
            (self.if_true, worry_level)
        } else {
            (self.if_false, worry_level)
        }
    }
}
