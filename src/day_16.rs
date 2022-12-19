use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let valves = read_lines(16).iter().map(|line| parse_line(line)).fold(
        HashMap::new(),
        |mut sum, valve| {
            sum.insert(valve.name.clone(), valve);
            sum
        },
    );

    // let volcano = Vulcano {
    //     valves,
    //     minutes_left: 30,
    // };
    // volcano.valves.len()

    let tmp = find_most_effective(&valves, &["AA".to_owned()], 30);

    println!("The most effective: {:?}", tmp.unwrap());
    0
}

type Valves = HashMap<String, Valve>;

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: usize,
    leads_to: Vec<String>,
    is_open: bool,
}

// struct Vulcano {
//     valves: Valves,
//     minutes_left: usize,
// }

// impl Vulcano {
//     fn tick(&mut self) {
//         //
//     }
// }

fn find_most_effective(
    valves: &Valves,
    visited: &[String],
    minutes: usize,
) -> Option<(Valve, usize)> {
    if minutes <= 1 {
        return None;
    }
    let current = valves.get(visited.last().unwrap()).unwrap();

    let mut tmp = vec![];

    // for loop_over = ;

    for valve in current
        .leads_to
        .iter()
        .map(|v| valves.get(v).unwrap())
        .filter(|v| !visited.contains(&v.name))
    {
        tmp.push(((*valve).clone(), valve.rate * (minutes - 1)));

        let mut visited = visited.to_vec();
        visited.push(valve.name.clone());

        if let Some(inner) = find_most_effective(valves, &visited, minutes - 1) {
            tmp.push(inner);
        }
    }

    // println!("======\n  {:?}", tmp);

    tmp.into_iter().max_by(|a, b| a.1.cmp(&b.1))

    // todo!()
}

fn parse_line(line: &str) -> Valve {
    let template =
        Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();
    let captures = template.captures(line).unwrap();

    let leads_to = captures[3]
        .split(", ")
        .map(|valve| valve.to_owned())
        .collect_vec();

    Valve {
        is_open: false,
        leads_to,
        name: captures[1].to_owned(),
        rate: captures[2].parse::<usize>().unwrap(),
    }
}
