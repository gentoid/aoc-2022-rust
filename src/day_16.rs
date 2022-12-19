use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let input = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
        .lines();
    let valves = input
        .map(|line| parse_line(line))
        .fold(HashMap::new(), |mut sum, valve| {
            sum.insert(valve.name.clone(), valve);
            sum
        });

    let actions = find_most_effective(&valves, 30, State::new("AA"));

    println!("The most effective: {:?}", actions);
    actions.1
}

type Valves = HashMap<String, Valve>;

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: usize,
    leads_to: Vec<String>,
    is_open: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum ActionType {
    Move,
    Open,
}

type Action = (ActionType, String);

#[derive(Clone, Default)]
struct State {
    current: String,
    opened: Vec<String>,
    since_last_opened: Vec<String>,
    actions: Vec<Action>,
}

impl State {
    fn new(init: &str) -> Self {
        Self {
            current: init.to_owned(),
            ..Default::default()
        }
    }

    fn move_to(&mut self, dest: &str) {
        self.current = dest.to_owned();
        self.actions.push((ActionType::Move, dest.to_owned()));
        self.since_last_opened.push(dest.to_owned());
    }

    fn open(&mut self, valve: &str) {
        self.opened.push(valve.to_owned());
        self.actions.push((ActionType::Open, valve.to_owned()));
        self.since_last_opened = vec![valve.to_owned()];
    }
}

fn find_most_effective(valves: &Valves, minutes: usize, state: State) -> (Vec<Action>, usize) {
    if state.actions.len() + 1 >= minutes {
        return (
            state.actions.to_vec(),
            calculate_flow(valves, minutes, &state.actions),
        );
    }

    let current = valves.get(&state.current).unwrap();

    let mut variants = vec![];

    for valve_name in current
        .leads_to
        .iter()
        .filter(|name| !state.since_last_opened.contains(name))
    {
        let valve = valves.get(valve_name).unwrap();

        let mut state = state.clone();
        state.move_to(&valve_name);

        if !state.opened.contains(&valve_name) && valve.rate > 0 {
            let mut state = state.clone();
            state.open(&valve_name);

            variants.push(find_most_effective(valves, minutes, state));
        }

        variants.push(find_most_effective(valves, minutes, state));
    }

    variants
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap_or((vec![], 0))
}

fn calculate_flow(valves: &Valves, minutes: usize, actions: &[Action]) -> usize {
    let mut add = 0;
    let mut output = 0;

    for action in actions {
        output += add;

        if action.0 == ActionType::Move {
            continue;
        }

        let valve = valves.get(&action.1).unwrap();
        add += valve.rate;
    }

    if minutes > actions.len() {
        return output + add * (minutes - actions.len());
    }

    output
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
