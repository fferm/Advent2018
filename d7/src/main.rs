extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Step {
    id: char,
    pre: Vec<char>
}

fn main() {
//    let filename = "input_small.txt";
    let filename = "input.txt";

    let mut inputs = read_inputs(filename);
    //let afters = analyze_after(&inputs);

    print_inputs(&inputs);

    while inputs.len() > 0 {
        let steps_without_pre = find_steps_without_pre(&inputs);
        let current_id = steps_without_pre.get(0).unwrap();
        print!("{}", current_id);
        take_one_out(*current_id, &mut inputs);
    }

    println!();

}

fn print_inputs(inputs: &HashMap<char, Step>) {
    for (id, step) in inputs {
        println!("{:?}", step);
    }
}
fn take_one_out(id_to_take_out: char, inputs: &mut HashMap<char, Step>) {
    inputs.remove(&id_to_take_out);

    for (id, step) in inputs {
        if step.pre.contains(&id_to_take_out) {
            let idx = step.pre.iter().position(|&i| i == id_to_take_out).unwrap();
            step.pre.remove(idx);
        }
    }

}

fn find_steps_without_pre(inputs: &HashMap<char, Step>) -> Vec<char> {
    let mut ret = Vec::new();
    for (id, step) in inputs {
        let pre = &step.pre;

        if pre.len() == 0 {
            ret.push(id.clone());
        }
    }

    ret.sort();

    return ret;
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
            let current_step = ret.entry(current_step_id).or_insert(Step{id: current_step_id, pre: Vec::new()});
            current_step.pre.push(current_predecessor_id);
        }

        ret.entry(current_predecessor_id).or_insert(Step{id: current_predecessor_id, pre: Vec::new()});
    }

    return ret;
}

fn analyze_after(inputs: &HashMap<char, Step>) -> HashMap<char, Vec<char>> {
    let mut ret: HashMap<char, Vec<char>> = HashMap::new();

    for (current_id, current_step) in inputs {
        for pre_id in current_step.pre.clone() {
            let mut afters_vec = ret.entry(pre_id).or_insert(Vec::new());
            (*afters_vec).push((*current_id).clone());
        }
    }

    return ret;
}

