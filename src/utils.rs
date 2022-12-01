use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

pub fn read_lines(day_number: usize) -> Vec<String> {
    let file = File::open(format!("inputs/{:02}.txt", day_number)).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn read_input_to_string(day_number: usize) -> String {
    read_to_string(format!("inputs/{:02}.txt", day_number))
        .expect(&format!("Tried to read {day_number}"))
}
