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
Valve JJ has flow rate=21; tunnel leads to valve II";
    let mut valves =
        input
            .lines()
            .map(|line| parse_line(line))
            .fold(HashMap::new(), |mut sum, valve| {
                sum.insert(valve.name.clone(), valve);
                sum
            });

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

    let tmp = find(&vec![], &valves, 30, "AA", &[], &[]);

    println!("{:?}", tmp);

    calculate_flow(30, &tmp.unwrap().1, true);

    0
}

type Valves = HashMap<String, Valve>;

#[derive(Clone)]
enum Action {
    Move(Link),
    Open(Valve),
}

impl Action {
    fn name(&self) -> String {
        match self {
            Action::Move(link) => link.to.clone(),
            Action::Open(valve) => valve.name.clone(),
        }
    }

    fn minutes(&self) -> usize {
        match self {
            Action::Move(link) => link.length,
            Action::Open(_) => 1,
        }
    }

    fn to_string(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move(link) => write!(f, "Move({}, {})", link.to, link.length),
            Action::Open(valve) => write!(f, "Open({}, {})", valve.name, valve.rate),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string(f)
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string(f)
    }
}

#[derive(Clone)]
struct Link {
    to: String,
    length: usize,
}

impl std::fmt::Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Link({}, {})", self.to, self.length)
    }
}

#[derive(Clone)]
struct Valve {
    name: String,
    rate: usize,
    links: Vec<Link>,
}

impl std::fmt::Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Valve({}, {}, {:?})", self.name, self.rate, self.links)
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

fn go(
    valves: Valves,
    from_name: &str,
    visited: &[String],
    opened: &[String],
) -> Vec<(Vec<Action>, Valves, Vec<String>, Vec<String>)> {
    let mut updated_valves = valves.clone();
    let Some(from) = updated_valves.remove(from_name) else {
    // println!("No key: {from_name} in {:?}", valves.keys());
        // println!("{:?}", valves.keys());
        return vec![];
    };
    let mut output = vec![];

    // println!("");

    for (i1, l1) in from.links.iter().enumerate() {
        if let Some(v) = updated_valves.get_mut(&l1.to) {
            v.links.retain(|l| l.to != from_name);
        }

        for (i2, l2) in from.links.iter().enumerate() {
            if i1 == i2 {
                continue;
            }

            let new_length = l1.length + l2.length;
            if let Some(v) = updated_valves.get_mut(&l1.to) {
                if let Some(link) = v.links.iter_mut().find(|l| l.to == l2.to) {
                    link.length = link.length.min(new_length);
                } else {
                    v.links.push(Link {
                        to: l2.to.clone(),
                        length: new_length,
                    });
                }
            }
        }
    }

    // println!("For {from_name} ppdated: {:?}", updated_valves);

    for link in &from.links {
        // println!("  Link to {:?}", link);

        let valve = valves.get(&link.to).unwrap();

        if !opened.contains(&valve.name) {
            let mut actions = vec![];
            actions.push(Action::Move(link.clone()));
            let mut opened = opened.to_vec();
            if valve.rate > 0 {
                actions.push(Action::Open(valve.clone()));
                opened.push(valve.name.to_owned());
            }
            output.push((actions, updated_valves.clone(), vec![], opened));
        }

        // TODO : improve it to not go same way twice
        if !visited.contains(&link.to) {
            let mut visited = visited.to_vec();
            visited.push(from_name.to_owned());
            output.push((
                vec![Action::Move(link.clone())],
                valves.clone(),
                visited,
                opened.to_vec(),
            ));
        }
    }

    output
}

fn find(
    path: &[Action],
    valves: &Valves,
    minutes_left: usize,
    from_name: &str,
    visited: &[String],
    opened: &[String],
) -> Option<(usize, Vec<Action>)> {
    if minutes_left <= 1 || valves.len() <= 1 {
        if path.is_empty() {
            // println!("No path detected");
            return None;
        }

        // println!("Path is too long: {:?}", path);
        let calc = calculate_flow(30, path, false);
        // println!(
        //     "{calc}, {:?}",
        //     path.iter()
        //         .filter(|l| match l {
        //             Action::Open(_) => true,
        //             _ => false,
        //         })
        //         .collect_vec()
        // );
        return Some((calc, path.to_vec()));
    }

    go(valves.clone(), from_name, &visited, &opened)
        .into_iter()
        .filter(|(actions, _, _, _)| take_minutes(actions) < minutes_left)
        .filter_map(|(actions, valves, visited, opened)| {
            let mut path = path.clone().to_vec();
            path.extend(actions.clone());
            find(
                &path,
                &valves,
                minutes_left - take_minutes(&actions) - 1,
                &actions.last().unwrap().name(),
                &visited,
                &opened,
            )
        })
        .max_by(|(a, _), (b, _)| a.cmp(b))
}

fn take_minutes(actions: &[Action]) -> usize {
    actions.iter().map(|a| a.minutes()).sum()
}

fn calculate_flow(minutes: usize, actions: &[Action], debug: bool) -> usize {
    let mut add = 0;
    let mut output = 0;

    if debug {
        let path = actions.iter().map(|a| format!("{a}")).join(" => ");
        println!("\nActions: {:?}", path);
    }

    let mut left_minutes = minutes;
    for action in actions {
        if debug {
            println!("{action}");
        }

        if left_minutes < action.minutes() {
            break;
        }

        left_minutes -= action.minutes();

        match action {
            Action::Move(_) => {
                output += add * action.minutes();
            }
            Action::Open(valve) => {
                output += add;
                add += valve.rate;
            }
        }

        if debug {
            println!("  After:   minutes left: {left_minutes},  add: {add},  output: {output}",);
        }
    }

    if left_minutes > 0 {
        output += add * left_minutes;
        left_minutes = 0;
    }

    if debug {
        println!("  End:     minutes left: {left_minutes},  add: {add},  output: {output}");
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
