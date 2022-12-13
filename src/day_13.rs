use std::fmt;

use crate::utils::{read_input_split_by_lines_number, read_lines};
use itertools::Itertools;

pub fn part_1() -> usize {
    read_input_split_by_lines_number(13, 3)
        .iter()
        .map(|input| parse_pair(input))
        .map(|(left, right)| compare(&left, &right))
        .enumerate()
        .filter(|(_, comparison)| comparison == &Comparison::Ok)
        .map(|(index, _)| index + 1)
        .sum()
}

pub fn part_2() -> usize {
    let input = read_lines(13);
    let (two_value, _) = parse("[[2]]");
    let (six_value, _) = parse("[[6]]");

    let mut tmp = input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| parse(line))
        .map(|(value, _)| value)
        .collect_vec();

    tmp.push(two_value.clone());
    tmp.push(six_value.clone());

    let tmp = sort(tmp);

    get_value_index(&two_value, &tmp) * get_value_index(&six_value, &tmp)
}

fn get_value_index(lookup_value: &Value, list: &[Value]) -> usize {
    let (index, _) = list
        .iter()
        .enumerate()
        .find(|(_, value)| *value == lookup_value)
        .unwrap();

    index + 1
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Number(usize),
    List(Box<Vec<Value>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::List(list) => {
                let formatted = list.iter().map(|item| format!("{item}")).join(", ");
                write!(f, "[{formatted}]")
            }
            Self::Number(num) => write!(f, "{num}"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Comparison {
    Ok,
    NotOk,
    Next,
}

fn parse_pair(input: &str) -> (Value, Value) {
    let lines = input.lines().collect_vec();

    let left = parse(lines[0]);
    let right = parse(lines[1]);

    (left.0, right.0)
}

fn parse(input: &str) -> (Value, usize) {
    if input.starts_with('[') {
        return get_list(input);
    }

    let mut inspected = 0;

    let mut number = String::from("");
    for (index, char) in input.char_indices() {
        match char {
            '0'..='9' => number.push(char),
            _ => {
                inspected = index;
                break;
            }
        }
    }

    let value = Value::Number(number.parse::<usize>().unwrap());

    (value, inspected)
}

fn get_list(input: &str) -> (Value, usize) {
    let mut inspected = 1;

    let mut output = Box::new(vec![]);

    if input.starts_with("[]") {
        inspected += 1;
        return (Value::List(output), inspected);
    }

    loop {
        let (value, inspected_inner) = parse(&input[inspected..]);
        output.push(value);
        inspected += inspected_inner;

        if &input[inspected..inspected + 1] == "]" {
            inspected += 1;
            break;
        }

        inspected += 1;
    }

    (Value::List(output), inspected)
}

fn compare(left: &Value, right: &Value) -> Comparison {
    use Comparison::*;
    use Value::*;

    match (left, right) {
        (Number(left), Number(right)) => {
            if left < right {
                Ok
            } else if left > right {
                NotOk
            } else {
                Next
            }
        }
        (List(left), List(right)) => {
            for (left, right) in left.iter().zip(right.iter()) {
                let result = compare(left, right);

                if result == Ok || result == NotOk {
                    return result;
                }
            }

            compare(&Number(left.len()), &Number(right.len()))
        }
        (List(_), Number(_)) => compare(left, &List(Box::new(vec![right.clone()]))),
        (Number(_), List(_)) => compare(&List(Box::new(vec![left.clone()])), right),
    }
}

fn sort(mut list: Vec<Value>) -> Vec<Value> {
    let len = list.len();

    if len == 0 || len == 1 {
        return list;
    }

    if len == 2 {
        return match compare(&list[0], &list[1]) {
            Comparison::NotOk => {
                list.reverse();
                list
            }
            _ => list,
        };
    }

    let pivot = list.remove(len / 2);

    let mut less = vec![];
    let mut more = vec![];

    for value in list {
        match compare(&value, &pivot) {
            Comparison::NotOk => {
                more.push(value);
            }
            _ => less.push(value),
        }
    }

    let mut sorted = sort(less);
    sorted.push(pivot);
    sorted.extend(sort(more));

    sorted
}
