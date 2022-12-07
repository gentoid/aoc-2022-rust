use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn read_lines(day_number: usize) -> Vec<String> {
    let file = File::open(format!("inputs/{:02}.txt", day_number)).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn read_input_to_string(day_number: usize) -> String {
    read_to_string(format!("inputs/{:02}.txt", day_number))
        .expect(&format!("Tried to read {day_number}"))
}

pub fn string_to_lines(str: &str) -> Vec<String> {
    str.lines().map(|line| line.to_owned()).collect_vec()
}
