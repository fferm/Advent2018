extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::cell::Cell;
use std::hash::{Hash, Hasher};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Step {
    id: char,
    pre: Vec<char>,
    required_time: i32,
    worked_on: Cell<bool>
}

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.pre.hash(state);
        self.required_time.hash(state);
        self.worked_on.get().hash(state);
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct WorkerStatus {
    id: i32,
    is_working: Cell<bool>,
    working_on_id: Cell<char>,
    time_left: Cell<i32>
}

impl Hash for WorkerStatus {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
        self.is_working.get().hash(state);
        self.working_on_id.get().hash(state);
        self.time_left.get().hash(state);
    }
}

const NON_WORKING_ID:char = '.';

fn main() {
    let small_run = false;

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
        &workers_data.push(WorkerStatus{id: worker_idx, is_working: Cell::new(false), working_on_id: Cell::new(NON_WORKING_ID), time_left: Cell::new(0)});
    }


    let mut current_second = 0;
    while inputs.len() > 0 {

        let mut ids_to_take_out = Vec::new();
        for worker_idx in 0..number_of_workers {

                let current_worker = (&mut workers_data).get(worker_idx as usize).unwrap();

                // Start of second
                if !current_worker.is_working.get() {
                    start_something_new(&current_worker, &mut inputs);
                }



                // In second
                if current_worker.is_working.get() {
                    current_worker.time_left.set(current_worker.time_left.get() - 1);
                }




                // End of second
                if current_worker.is_working.get() && current_worker.time_left.get() == 0 {
                    ids_to_take_out.push(current_worker.working_on_id.get());
                    current_worker.is_working.set(false);
                }
        }

        print_second(current_second, &workers_data);


        for id in ids_to_take_out {
            take_one_out(id, &mut inputs);
        }

        current_second = current_second + 1;

    }


    println!("Completed in {} seconds", current_second);

}

fn print_second(current_second: i32, workers_data: &Vec<WorkerStatus>) {
    print!("sec: {:3}\t", current_second);
    for current_worker in workers_data {
        print!("{}\t", current_worker.working_on_id.get());
    }
    println!();

}

fn start_something_new(current_worker: &WorkerStatus, inputs: &mut HashMap<char, Step>) {
    let steps_without_pre = find_steps_without_pre(&inputs);

    if steps_without_pre.len() > 0 {
        let mut current_id= NON_WORKING_ID;

        for id in steps_without_pre {
            let step = (&inputs).get(&id).unwrap();

            if !step.worked_on.get() {
                step.worked_on.set(true);
                current_id = id;
                break;
            }
        }

        current_worker.working_on_id.set(current_id);
        if current_id != NON_WORKING_ID {
            current_worker.is_working.set(true);
            current_worker.time_left.set((&inputs).get(&current_id).unwrap().required_time);
        } else {
            current_worker.is_working.set(false);
        }
    }
}

fn print_inputs(inputs: &HashMap<char, Step>) {
    for (_id, step) in inputs {
        println!("{:?}", step);
    }
}
fn take_one_out(id_to_take_out: char, inputs: &mut HashMap<char, Step>) {
    inputs.remove(&id_to_take_out);

    for (_id, step) in inputs {
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
            let current_step = ret.entry(current_step_id).or_insert(
                Step{
                    id: current_step_id,
                    pre: Vec::new(),
                    required_time: required_time_for_step(current_step_id, additional_seconds_per_task),
                    worked_on: Cell::new(false)});
            current_step.pre.push(current_predecessor_id);
        }

        ret.entry(current_predecessor_id).or_insert(
            Step{
                id: current_predecessor_id,
                pre: Vec::new(),
                required_time: required_time_for_step(current_predecessor_id, additional_seconds_per_task),
                worked_on: Cell::new(false)});
    }

    return ret;
}

fn required_time_for_step(id: char, additional_time: i32) -> i32 {
    let ascii = id as i32;

    return additional_time + ascii - 64;
}
