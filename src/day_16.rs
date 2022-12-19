use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> usize {
    let mut valves = read_lines(16).iter().map(|line| parse_line(line)).fold(
        HashMap::new(),
        |mut sum, valve| {
            sum.insert(valve.name.clone(), valve);
            sum
        },
    );

    let valves_clone = valves.clone();

    let useless = valves_clone
        .values()
        .filter(|v| v.links.len() == 2 && v.rate == 0)
        .collect_vec();

    for v in useless {
        let tmp = valves.get(&v.name).unwrap().clone();

        valves = optimize(valves, &tmp.links[0].to, &v.name, &tmp.links[1]);
        valves = optimize(valves, &tmp.links[1].to, &v.name, &tmp.links[0]);

        valves.remove(&v.name);
    }

    for v in &valves {
        println!("{:?}", v);
    }

    // let mut counter = 0;
    let mut tmp = go(valves, "AA");

    loop {
        if tmp.len() == 0 {
            break;
        }

        // counter += 1;

        let (link, v) = tmp[0].clone();

        println!("======");
        println!("Go: {:?}", link);

        for val in v.values() {
            println!("{:?}", val);
        }

        tmp = go(v, &link.to);
    }

    // let actions = find_most_effective(&valves, 30, State::new("AA"));

    // println!("The most effective: {:?}", actions);
    // actions.1
    0
}

type Valves = HashMap<String, Valve>;

#[derive(Clone, Debug)]
struct Link {
    to: String,
    length: usize,
}

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: usize,
    links: Vec<Link>,
}

#[derive(Clone, Debug, PartialEq)]
enum ActionType {
    Move,
    Open,
}

type Action = (ActionType, String, usize);

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

    fn move_to(&mut self, link: &Link) {
        self.current = link.to.to_owned();
        self.actions
            .push((ActionType::Move, link.to.to_owned(), link.length));
        self.since_last_opened.push(link.to.to_owned());
    }

    fn open(&mut self, valve: &str) {
        self.opened.push(valve.to_owned());
        self.actions.push((ActionType::Open, valve.to_owned(), 1));
        self.since_last_opened = vec![valve.to_owned()];
    }
}

fn optimize(mut valves: Valves, node: &str, from: &str, to: &Link) -> Valves {
    // println!("Reconnect: {node} from {from} to {}", to.to);
    let mut link = valves
        .get_mut(node)
        .unwrap()
        .links
        .iter_mut()
        .find(|v| v.to == from)
        .unwrap();
    link.to = to.to.clone();
    link.length += to.length;

    valves
}

fn go(mut valves: Valves, from_name: &str) -> Vec<(Link, Valves)> {
    let from = valves.remove(from_name).unwrap();
    let mut output = vec![];

    for (i1, v1) in from.links.iter().enumerate() {
        let mut valves = valves.clone();

        if !valves.contains_key(&v1.to) {
            continue;
        }

        let links = &mut valves.get_mut(&v1.to).unwrap().links;
        links.retain(|l| l.to != from_name);

        for (i2, v2) in from.links.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            links.push(Link {
                to: v2.to.clone(),
                length: v1.length + v2.length,
            });
        }

        output.push((v1.clone(), valves));
    }

    output
}

fn find_most_effective(valves: &Valves, minutes: usize, state: State) -> (Vec<Action>, usize) {
    let counter: usize = state.actions.iter().map(|a| a.2).sum();
    if counter + 1 >= minutes {
        return (
            state.actions.to_vec(),
            calculate_flow(valves, minutes, &state.actions),
        );
    }

    let current = valves.get(&state.current).unwrap();

    let mut variants = vec![];

    for link in current
        .links
        .iter()
        .filter(|link| !state.since_last_opened.contains(&link.to))
    {
        let valve = valves.get(&link.to).unwrap();

        let mut state = state.clone();
        state.move_to(&link);

        if !state.opened.contains(&link.to) && valve.rate > 0 {
            let mut state = state.clone();
            state.open(&link.to);

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

    let mut counter = 0;
    for action in actions {
        if action.2 + counter > minutes {
            output += add * (minutes - counter);
            counter += minutes - counter;
            break;
        } else {
            output += add * action.2;
            counter += action.2;
        }

        if action.0 == ActionType::Move {
            continue;
        }

        let valve = valves.get(&action.1).unwrap();
        add += valve.rate;
    }

    if minutes > counter {
        return output + add * (minutes - counter);
    }

    output
}

fn parse_line(line: &str) -> Valve {
    let template =
        Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();
    let captures = template.captures(line).unwrap();

    let leads_to = captures[3]
        .split(", ")
        .map(|valve| Link {
            to: valve.to_owned(),
            length: 1,
        })
        .collect_vec();

    Valve {
        links: leads_to,
        name: captures[1].to_owned(),
        rate: captures[2].parse::<usize>().unwrap(),
    }
}
