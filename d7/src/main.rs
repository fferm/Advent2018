extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Step {
    id: char,
    pre: Vec<char>,
    required_time: i32
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct WorkerStatus {
    id: i32,
    is_working: bool,
    working_on_id: char,
    time_left: i32
}

fn main() {
    let small_run = true;

    let filename;
    let additional_seconds_per_task;
    let number_of_workers;

    if small_run {
        filename = "input_small.txt";
        additional_seconds_per_task = 0;
        number_of_workers = 2;
    } else {
        filename = "input.txt";
        additional_seconds_per_task = 60;
        number_of_workers = 5;
    }

    let mut inputs = read_inputs(filename, additional_seconds_per_task);

    print_inputs(&inputs);

    let mut workers_data = Vec::new();
    for worker_idx in 0..number_of_workers {

        workers_data.push(WorkerStatus{id: worker_idx, is_working: false, working_on_id: '0', time_left: 0});
    }


    let mut current_second = 0;


//    while inputs.len() > 0 {
//        let steps_without_pre = find_steps_without_pre(&inputs);
//        let current_id = steps_without_pre.get(0).unwrap();
//        print!("{}", current_id);
//        take_one_out(*current_id, &mut inputs);
//    }
//
//    println!();

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

fn read_inputs(filename: &str, additional_seconds_per_task: i32) -> HashMap<char, Step> {
    let mut ret = HashMap::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    for line in lines {
        let re = Regex::new("Step (\\w) must be finished before step (\\w) can begin.").unwrap();
        let cap = re.captures_iter(line).next().expect("Error in capturing regex");

        let current_step_id = cap[2].chars().next().unwrap();
        let current_predecessor_id = cap[1].chars().next().unwrap();

        {
            let current_step = ret.entry(current_step_id).or_insert(Step{id: current_step_id, pre: Vec::new(), required_time: required_time_for_step(current_step_id, additional_seconds_per_task)});
            current_step.pre.push(current_predecessor_id);
        }

        ret.entry(current_predecessor_id).or_insert(Step{id: current_predecessor_id, pre: Vec::new(), required_time: required_time_for_step(current_predecessor_id, additional_seconds_per_task)});
    }

    return ret;
}

fn required_time_for_step(id: char, additional_time: i32) -> i32 {
    let ascii = id as i32;

    return additional_time + ascii - 64;
}
