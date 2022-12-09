use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> u32 {
    for line in read_lines(7).iter().map(parse_line) {
        println!("{:?}", line);
    }

    0
}

#[derive(Debug)]
enum Line {
    Cmd(Command),
    Output(OutputLine),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum OutputLine {
    File(String, u32),
    Dir(String),
}

fn parse_line(line: &String) -> Line {
    use Line::*;

    match &line[0..2] {
        "$ " => Cmd(parse_command(&line[2..])),
        _ => Output(parse_output(&line)),
    }
}

fn parse_command(line: &str) -> Command {
    use Command::*;

    match &line[0..2] {
        "cd" => Cd(line[3..].to_owned()),
        "ls" => Ls,
        _ => unreachable!(),
    }
}

fn parse_output(line: &str) -> OutputLine {
    use OutputLine::*;

    if line.starts_with("dir ") {
        return Dir(line[4..].to_owned());
    }

    let template = Regex::new(r"^(\d+) (.+)$").unwrap();
    let captures = template.captures(line).unwrap();

    File(captures[2].to_owned(), captures[1].parse::<u32>().unwrap())
}
