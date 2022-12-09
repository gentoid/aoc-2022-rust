use std::collections::HashMap;

use regex::Regex;

use crate::utils::read_lines;

pub fn part_1() -> u32 {
    let mut fs = Fs::new();

    for line in read_lines(7).iter().map(parse_line) {
        fs.process_line(line);
    }

    fs.dirs
        .iter()
        .map(|dir| dir.size(&fs))
        .filter(|size| *size <= 100000).sum()
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
    File(File),
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
    if line.starts_with("dir ") {
        return OutputLine::Dir(line[4..].to_owned());
    }

    let template = Regex::new(r"^(\d+) (.+)$").unwrap();
    let captures = template.captures(line).unwrap();

    OutputLine::File(File {
        name: captures[2].to_owned(),
        size: captures[1].parse::<u32>().unwrap(),
    })
}

#[derive(Debug)]
struct Fs {
    dirs: Vec<Dir>,
    indexed_path: Vec<usize>,
}

impl Fs {
    pub fn new() -> Self {
        Self {
            dirs: vec![Dir::root()],
            indexed_path: vec![0],
        }
    }

    pub fn process_line(&mut self, line: Line) {
        match line {
            Line::Cmd(cmd) => self.process_cmd(cmd),
            Line::Output(output) => self.process_output(output),
        }
    }

    fn process_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Cd(path) => self.process_cd_cmd(&path),
            Command::Ls => return,
        }
    }

    fn process_cd_cmd(&mut self, path: &str) {
        match path {
            "/" => self.indexed_path = vec![0],
            ".." => {
                if self.indexed_path.len() > 1 {
                    self.indexed_path.pop();
                }
            }
            _ => {
                let current_dir_index = self.indexed_path.last().unwrap();

                match self.dirs[*current_dir_index].dirs.get(path) {
                    None => println!("There's no such a nested dir: {path}"),
                    Some(dir_index) => {
                        self.indexed_path.push(*dir_index);
                    }
                }
            }
        }
    }

    fn process_output(&mut self, output: OutputLine) {
        match output {
            OutputLine::Dir(dir_name) => {
                let current_dir_index = self.indexed_path.last().unwrap();

                self.dirs.push(Dir {
                    name: dir_name.to_owned(),
                    parent_dir_index: Some(*current_dir_index),
                    ..Default::default()
                });

                let dir_index = self.dirs.len() - 1;

                self.dirs[*current_dir_index]
                    .dirs
                    .insert(dir_name.to_owned(), dir_index);
            }
            OutputLine::File(file) => {
                let current_dir_index = self.indexed_path.last().unwrap();

                self.dirs[*current_dir_index].files.push(file);
            }
        }
    }
}

#[derive(Debug, Default)]
struct Dir {
    name: String,
    parent_dir_index: Option<usize>,
    files: Vec<File>,
    dirs: HashMap<String, usize>,
}

impl Dir {
    pub fn root() -> Self {
        Self {
            name: "/".to_owned(),
            ..Default::default()
        }
    }

    pub fn size(&self, fs: &Fs) -> u32 {
        let files_sizes: u32 = self.files.iter().map(|file| file.size).sum();
        let dirs_sizes: u32 = self
            .dirs
            .values()
            .map(|dir_index| fs.dirs[*dir_index].size(fs))
            .sum();

        files_sizes + dirs_sizes
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}
