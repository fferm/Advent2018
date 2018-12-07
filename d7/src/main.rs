extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Step {
    id: char,
    pre: Vec<char>,
    after: Vec<char>
}

fn main() {
    let filename = "input_small.txt";
    // let filename = "input.txt";

    let inputs = read_inputs(filename);
    analyze_after(&inputs);

    for step in inputs {
        println!("{:?}", step);
    }
}

fn read_inputs(filename: &str) -> HashMap<char, Step> {
    let mut ret = HashMap::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    for line in lines {
        let re = Regex::new("Step (\\w) must be finished before step (\\w) can begin.").unwrap();
        let cap = re.captures_iter(line).next().expect("Error in capturing regex");

        let current_step_id = cap[2].chars().next().unwrap();
        let current_predecessor_id = cap[1].chars().next().unwrap();

        {
            let current_step = ret.entry(current_step_id).or_insert(Step{id: current_step_id, pre: Vec::new(), after: Vec::new()});
            current_step.pre.push(current_predecessor_id);
        }

        ret.entry(current_predecessor_id).or_insert(Step{id: current_predecessor_id, pre: Vec::new(), after: Vec::new()});
    }

    return ret;
}

fn analyze_after(inputs: &HashMap<char, Step>) {
    for (current_id, current_step) in inputs {
        for pre_id in current_step.pre.clone() {
            let mut pre_step = inputs.get(&pre_id).unwrap();
            let mut pre_after_list = pre_step.after;
            pre_after_list.push(current_id.clone());
        }
    }
}

