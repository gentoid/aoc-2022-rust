use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::utils::read_input_to_string;

pub fn part_1() -> usize {
    iterate(20, true)
}

pub fn part_2() -> usize {
    iterate(1000, false)
}

fn iterate(rounds: u32, divide_by_3: bool) -> usize {
    let check_rounds = vec![];
    // 1, 20, 100, 500, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000,
    // ];
    let mut monkeys = read_input_to_string(11)
        .split("\n\n")
        .map(|input| parse_monkey(input, divide_by_3))
        .collect_vec();

    let modulos = Modulos::new(&monkeys.iter().map(|m| m.divisor).collect_vec());

    let mut all_updates: HashMap<usize, Vec<ItemType>> = HashMap::new();

    for round in 0..rounds {
        if !divide_by_3 && check_rounds.contains(&round) {
            println!("\n=== AFTER ROUND {round} ===");
        }

        for (index, monkey) in monkeys.iter_mut().enumerate() {
            if !divide_by_3 && check_rounds.contains(&round) {
                println!("{index} inspected {}", monkey.inspected);
            }
            monkey.with_updates(all_updates.get_mut(&index));

            for (monkey, item) in monkey.turn() {
                all_updates.entry(monkey).or_default().push(item);
            }
        }
    }

    println!("====== THE END ======");

    for (index, monkey) in monkeys.iter_mut().enumerate() {
        monkey.with_updates(all_updates.get_mut(&index));
        println!("{index} inspected {}", monkey.inspected);
    }

    let mut items_inspected = monkeys.iter().map(|monkey| monkey.inspected).collect_vec();
    items_inspected.sort();
    items_inspected.reverse();

    items_inspected[0] * items_inspected[1]
}

fn parse_monkey(input: &str, divide_by_3: bool) -> Monkey {
    let (if_true, if_false) = parse_throws(input);

    Monkey {
        items: parse_items(input),
        operation: parse_operation(input),
        divisor: parse_test(input),
        if_true,
        if_false,
        inspected: 0,
        divide_by_3,
    }
}

fn parse_items(input: &str) -> Vec<ItemType> {
    let template = Regex::new(r"Starting items: ((?:\d+, )*\d+)").unwrap();
    let captures = template.captures(input).unwrap();
    captures[1]
        .split(", ")
        .map(|num| num.parse::<ItemType>().unwrap())
        .collect_vec()
}

fn parse_operation(input: &str) -> Box<dyn Fn(Modulos) -> ()> {
    let template = Regex::new(r"Operation: new = old ([+*]) ((:?\d+)|(:?\w+))").unwrap();
    let captures = template.captures(input).unwrap();

    if &captures[2] == "old" {
        return match &captures[1] {
            "*" => Box::new(move |_i| ()),
            _ => unreachable!(),
        };
    }

    let value = captures[2].parse::<ItemType>().unwrap();

    match &captures[1] {
        "+" => Box::new(move |i| i.update(value, Operation::Sum)),
        "*" => Box::new(move |i| i.update(value, Operation::Sum)),
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

enum Operation {
    Sum,
    Multiply,
}

#[derive(Clone)]
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

    fn update(&mut self, update_with: ItemType, operation: Operation) {
        for (key, value) in self.data.iter_mut() {
            match operation {
                Operation::Sum => *value = (value + update_with) % key,
                Operation::Multiply => *value = (value * update_with) % key,
            }
        }
    }
}

struct Monkey {
    items: Vec<Modulos>,
    operation: Box<dyn Fn(Modulos) -> ()>,
    divisor: ItemType,
    if_true: usize,
    if_false: usize,
    inspected: usize,
    divide_by_3: bool,
}

impl Monkey {
    fn with_updates(&mut self, items: Option<&mut Vec<Modulos>>) {
        if let Some(items) = items {
            self.items.extend(items.clone());
            *items = vec![];
        };
    }

    fn turn(&mut self) -> Vec<(usize, ItemType)> {
        let output = self
            .items
            .iter()
            .map(|item| self.inspect(*item))
            .collect_vec();
        self.items = vec![];
        self.inspected += output.len();

        output
    }

    fn inspect(&self, item: Modulos) -> (usize, ItemType) {
        (self.operation)(item);
        // let mut worry_level = item.

        if self.divide_by_3 {
            worry_level /= 3;
        }

        if item.data.get(&self.divisor).unwrap() == &0 {
            (self.if_true, worry_level)
        } else {
            (self.if_false, worry_level)
        }
    }
}
