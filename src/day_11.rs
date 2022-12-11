use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::utils::read_input_to_string;

// pub fn part_1() -> usize {
//     iterate(20, true)
// }

pub fn part_2() -> usize {
    iterate(10000)
}

fn iterate(rounds: u32) -> usize {
    let (mut monkeys, items): (Vec<_>, Vec<_>) = read_input_to_string(11)
        .split("\n\n")
        .map(|input| parse_monkey(input))
        .unzip();

    let divisors = monkeys.iter().map(|m| m.divisor).collect_vec();

    for (index, items) in items.iter().enumerate() {
        for item in items {
            let mut modulos = Modulos::new(&divisors);
            modulos.update(&(Operation::Sum, *item));
            monkeys[index].items.push(modulos);
        }
    }

    let mut all_updates: HashMap<usize, Vec<Modulos>> = HashMap::new();

    for _ in 0..rounds {
        for (index, monkey) in monkeys.iter_mut().enumerate() {
            monkey.with_updates(all_updates.get_mut(&index));

            for (monkey, item) in monkey.turn() {
                all_updates.entry(monkey).or_default().push(item);
            }
        }
    }

    for (index, monkey) in monkeys.iter_mut().enumerate() {
        monkey.with_updates(all_updates.get_mut(&index));
    }

    let mut items_inspected = monkeys.iter().map(|monkey| monkey.inspected).collect_vec();
    items_inspected.sort();
    items_inspected.reverse();

    items_inspected[0] * items_inspected[1]
}

fn parse_monkey(input: &str) -> (Monkey, Vec<ItemType>) {
    let (if_true, if_false) = parse_throws(input);

    let monkey = Monkey {
        items: vec![],
        operation: parse_operation(input),
        divisor: parse_test(input),
        if_true,
        if_false,
        inspected: 0,
    };

    (monkey, parse_items(input))
}

fn parse_items(input: &str) -> Vec<ItemType> {
    let template = Regex::new(r"Starting items: ((?:\d+, )*\d+)").unwrap();
    let captures = template.captures(input).unwrap();
    captures[1]
        .split(", ")
        .map(|num| num.parse::<ItemType>().unwrap())
        .collect_vec()
}

fn parse_operation(input: &str) -> (Operation, ItemType) {
    let template = Regex::new(r"Operation: new = old ([+*]) ((:?\d+)|(:?\w+))").unwrap();
    let captures = template.captures(input).unwrap();

    if &captures[2] == "old" {
        return match &captures[1] {
            "*" => (Operation::Power, 0),
            _ => unreachable!(),
        };
    }

    let value = captures[2].parse::<ItemType>().unwrap();

    match &captures[1] {
        "+" => (Operation::Sum, value),
        "*" => (Operation::Multiply, value),
        _ => unreachable!(),
    }
}

fn parse_test(input: &str) -> ItemType {
    let template = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let captures = template.captures(input).unwrap();

    captures[1].parse::<ItemType>().unwrap()
}

fn parse_throws(input: &str) -> (usize, usize) {
    let template =
        Regex::new(r"If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();
    let captures = template.captures(input).unwrap();

    let if_true = captures[1].parse::<usize>().unwrap();
    let if_false = captures[2].parse::<usize>().unwrap();

    (if_true, if_false)
}

type ItemType = usize;

#[derive(Debug)]
enum Operation {
    Sum,
    Multiply,
    Power,
}

#[derive(Clone, Debug)]
struct Modulos {
    data: HashMap<ItemType, ItemType>,
}

impl Modulos {
    fn new(divisors: &[ItemType]) -> Self {
        let mut data = HashMap::new();
        for div in divisors {
            data.insert(*div, 0);
        }

        Self { data }
    }

    fn update(&mut self, operation: &(Operation, ItemType)) {
        for (key, value) in self.data.iter_mut() {
            match &operation.0 {
                Operation::Sum => *value = (*value + operation.1) % key,
                Operation::Multiply => *value = (*value * operation.1) % key,
                Operation::Power => *value = (*value * *value) % key,
            }
        }
    }
}

struct Monkey {
    items: Vec<Modulos>,
    operation: (Operation, ItemType),
    divisor: ItemType,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

impl Monkey {
    fn with_updates(&mut self, items: Option<&mut Vec<Modulos>>) {
        if let Some(items) = items {
            self.items.extend(items.clone());
            *items = vec![];
        };
    }

    fn turn(&mut self) -> Vec<(usize, Modulos)> {
        let output = self
            .items
            .iter()
            .map(|item| self.inspect(item.clone()))
            .collect_vec();
        self.items = vec![];
        self.inspected += output.len();

        output
    }

    fn inspect(&self, mut item: Modulos) -> (usize, Modulos) {
        item.update(&self.operation);

        let throws_to = if *(item.data.get(&self.divisor).unwrap()) == 0 {
            self.if_true
        } else {
            self.if_false
        };

        (throws_to, item)
    }
}
