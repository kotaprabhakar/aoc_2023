use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

pub fn get_input_from_aoc(problem_number: i32) -> String {
    let client = reqwest::blocking::Client::new();
    client
        .get(format!("https://adventofcode.com/2023/day/{}/input", problem_number).as_str())
        .header(reqwest::header::COOKIE, 
                format!("session={}", crate::hash_code::get_hash_code()).as_str())
        .send()
        .expect("!!!Error in retrieving call to Advent Of Code!!!")
        .text()
        .expect("!!!Unable to retrieve text from call to Advent Of Code!!!")
        .trim()
        .to_string()
}

pub fn get_lines_from_string(file_as_string: String) -> Vec<String> {
    file_as_string.split('\n').map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn get_lines_from_file(file_name: &'static str) -> Vec<String> {
    let file = File::open(file_name).expect(format!("No such file- {}", file_name).as_str());
    let buf = BufReader::new(file);
    buf.lines()
       .map(|line| 
            line.expect(format!("Could not find line in file- {}", file_name).as_str()))
       .collect()
}