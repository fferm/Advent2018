extern crate regex;

use std::fs;
use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let mut small_input = true;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut program = read_inputs(filename);
    println!("{:?}", program);
}



fn read_inputs(filename: &str) -> Program {
    let mut steps = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();
    let mut current_line = 0;

    while current_line < lines.len() {
        let before_line = lines.get(current_line).unwrap();
        let before_register = read_register_line(before_line, "Before: ");

        let instruction_line = lines.get(current_line + 1).unwrap();
        let instruction_regex = "(\\d+) (\\d+) (\\d+) (\\d+)";
        let cap = Regex::new(&instruction_regex).unwrap().captures_iter(instruction_line).next().expect("Error in capturing instruction regex");
        let instruction = Instruction{op_code: cap[1].parse().unwrap(), a: cap[2].parse().unwrap(), b: cap[3].parse().unwrap(), c: cap[4].parse().unwrap()};

        let after_line = lines.get(current_line + 2).unwrap();
        let after_register = read_register_line(after_line, "After:  ");

        steps.push(Step{before: before_register, instruction: instruction, after: after_register});

        current_line += 4;
    }

    let mut program = Program {steps};
    return program;
}

fn read_register_line(line: &str, starting_text: &str) -> Registers {
    let regex = starting_text.to_owned() + "\\[(\\d+), (\\d+), (\\d+), (\\d+)\\]";
    let re = Regex::new(&regex[..]).unwrap();
    let cap = re.captures_iter(line).next().expect("Error in capturing regex");
    let register = Registers{r0: Cell::new(cap[1].parse().unwrap()), r1: Cell::new(cap[2].parse().unwrap()), r2: Cell::new(cap[3].parse().unwrap()), r3: Cell::new(cap[4].parse().unwrap())};

    return register;

}

#[derive(Debug)]
struct Program {
    steps: Vec<Step>
}

#[derive(Debug)]
struct Step {
    before: Registers,
    after: Registers,
    instruction: Instruction
}

#[derive(Debug)]
struct Registers {
    r0: Cell<isize>,
    r1: Cell<isize>,
    r2: Cell<isize>,
    r3: Cell<isize>
}

#[derive(Debug)]
struct Instruction {
    op_code: isize,
    a: isize,
    b: isize,
    c: isize
}