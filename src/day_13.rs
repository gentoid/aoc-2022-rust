use crate::utils::read_input_split_by_lines_number;
use itertools::Itertools;

pub fn part_1() -> u32 {
    let tmp = read_input_split_by_lines_number(13, 3)
        .iter()
        .map(|input| parse_input(input))
        .collect_vec();
    0
}

#[derive(Clone, Debug)]
enum Value {
    Number(usize),
    List(Vec<Box<Value>>),
}

#[derive(PartialEq)]
enum Comparison {
    Ok,
    NotOk,
    Next,
}

fn parse_input(input: &str) {
    let tmp = input.lines().map(|line| parse(line, 0)).collect_vec();

    println!("{:?}", tmp);
}

fn parse(input: &str, nested: usize) -> (Value, usize) {
    // println!("{}Parse:    {input}", " ".repeat(nested));
    if input.starts_with('[') {
        let value = get_list(input, nested + 2);
        // println!("{}Return parsed [list]:  {:?}", " ".repeat(nested), value);
        return value;
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
    // println!(
    //     "{}Return parsed [value]: ({:?}, {inspected})",
    //     " ".repeat(nested),
    //     value
    // );
    (value, inspected)
}

fn get_list(input: &str, nested: usize) -> (Value, usize) {
    // println!("{}Get list: {input}", " ".repeat(nested));
    let mut inspected = 1;

    let mut output = vec![];

    if input.starts_with("[]") {
        inspected += 1;
        // println!(
        //     "{}Return list [empty]:   ({:?}, {inspected})",
        //     " ".repeat(nested),
        //     output
        // );
        return (Value::List(output), inspected);
    }

    loop {
        // println!("{}Loop start at {inspected}", " ".repeat(nested));
        let (value, inspected_inner) = parse(&input[inspected..], nested + 2);
        output.push(Box::new(value));
        inspected += inspected_inner;

        if &input[inspected..inspected + 1] == "]" {
            // println!("{}End oof list at {}", " ".repeat(nested), inspected + 1);
            inspected += 1;
            break;
        }

        inspected += 1;
    }

    // println!(
    //     "{}Retur list [filled]: ({:?}, {})",
    //     " ".repeat(nested),
    //     output,
    //     inspected
    // );
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
        (List(_), Number(_)) => compare(left, &List(vec![Box::new(right.clone())])),
        (Number(_), List(_)) => compare(&List(vec![Box::new(left.clone())]), right),
    }
}
