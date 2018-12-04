extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let inputs = read_inputs("input.txt");
}

fn read_inputs(filename: &str) -> ??? {
    let mut ret = ???

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    for line in lines {
        ret.push(???);

    }

    return ret;
}
