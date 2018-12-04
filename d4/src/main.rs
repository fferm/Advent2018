extern crate regex;
extern crate bit_vec;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use bit_vec::BitVec;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Action {
    BEGIN,
    SLEEP,
    WAKE
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct InputData {
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    guard_id: i32,
    action: Action
}

#[derive(Debug, Hash, Copy, Clone, Eq)]
struct GuardDayDataIndex {
    month: i32,
    day: i32,
    guard_id: i32,
}
impl PartialEq for GuardDayDataIndex {
    fn eq(&self, other: &GuardDayDataIndex) -> bool {
        self.month == other.month && self.day == other.day && self.guard_id == other.guard_id
    }
}


fn main() {
    let inputs = read_inputs("input.txt");

    let sorted_inputs = sort_inputs(&inputs);

    let guard_datas = analyze_inputs(sorted_inputs);

    print_schedule(&guard_datas);

    let guard_with_most_minutes_asleep = find_guard_with_most_minutes_asleep(&guard_datas);
    let max_minute = find_minute_where_max_guard_sleeps_most(guard_with_most_minutes_asleep, &guard_datas);


    println!("Answer is {} * {} = {}", guard_with_most_minutes_asleep, max_minute, guard_with_most_minutes_asleep * max_minute);
}

fn read_inputs(filename: &str) -> Vec<InputData> {
    let mut ret = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    for line in lines {
//        println!("{}", line);
        let mut id: i32 = -1;
        let re = Regex::new("\\[\\d+-(\\d+)-(\\d+) (\\d+):(\\d+)\\] (Guard #(\\d+) begins shift|falls asleep|wakes up)").unwrap();
        let cap = re.captures_iter(line).next().unwrap();

        let month: i32 = cap[1].parse().unwrap();
        let day: i32 = cap[2].parse().unwrap();
        let hour: i32 = cap[3].parse().unwrap();
        let minute: i32 = cap[4].parse().unwrap();

        let action_text = &cap[5];
        let mut action: Action;
        if action_text == "falls asleep" {
            action = Action::SLEEP;
        } else if action_text == "wakes up" {
            action = Action::WAKE;
        } else {
            action = Action::BEGIN;
            id = cap[6].parse().unwrap();
        }

        let data = InputData { month: month, day: day, hour: hour, minute: minute, guard_id: id, action: action };


//        println!("{}\tdata{:?}", line, data);
        ret.push(data);

    }

    return ret;
}

fn sort_inputs(unsorted: &Vec<InputData>) -> Vec<InputData> {
    let mut ret = unsorted.to_vec();
    ret.sort();

    return ret;
}

fn analyze_inputs(inputs: Vec<InputData>) -> HashMap<GuardDayDataIndex, BitVec> {
    let mut guard_datas: HashMap<GuardDayDataIndex, BitVec> = HashMap::new();

    let mut current_guard = -1;
    for input_data in inputs {
        match input_data.action {
            Action::BEGIN => {
                current_guard = input_data.guard_id;
            },
            Action::SLEEP => {
                let minute = input_data.minute;
                let idx = GuardDayDataIndex { month: input_data.month, day: input_data.day, guard_id: current_guard };

                let current_guard_data = guard_datas.entry(idx).or_insert(new_guard_data());

                for current_minute in minute..59 {
                    current_guard_data.set(current_minute as usize, true);
                }
            },
            Action::WAKE => {
                let minute = input_data.minute;
                let idx = GuardDayDataIndex { month: input_data.month, day: input_data.day, guard_id: current_guard };

                let current_guard_data = guard_datas.entry(idx).or_insert(new_guard_data());

                for current_minute in minute..59 {
                    current_guard_data.set(current_minute as usize, false);
                }
            }
        }
    }

    return guard_datas;
}

fn print_schedule(guard_datas: &HashMap<GuardDayDataIndex, BitVec<u32>>) -> () {
    println!("DATE   ID    MINUTE");
    println!("             000000000011111111112222222222333333333344444444445555555555");
    println!("             012345678901234567890123456789012345678901234567890123456789");

    for (idx, data) in guard_datas {
        print!("{:02}-{:02}  #{:04} ", idx.month, idx.day, idx.guard_id);
        for minute in 0..59 {
            if data.get(minute).unwrap() {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!();
    }
}

fn find_guard_with_most_minutes_asleep(guard_datas: &HashMap<GuardDayDataIndex, BitVec<u32>>) -> i32 {
    let mut minutes_by_guard = HashMap::new();

    for (idx, data) in guard_datas {
        let current_guard = idx.guard_id;

        let mut current_guard_minutes = 0;
        for minute in 0..59 {
            if data.get(minute).unwrap() {
                current_guard_minutes = current_guard_minutes + 1;
            }
        }

        let current_minutes_by_guard = minutes_by_guard.entry(current_guard).or_insert(0);
        *current_minutes_by_guard = *current_minutes_by_guard + current_guard_minutes;
    }


    let mut max_guard = -1;
    let mut max_minutes = -1;
    for (guard, minutes) in minutes_by_guard {
        if minutes > max_minutes {
            max_guard = guard;
            max_minutes = minutes;
        }
    }

    println!("guard {} is max with {} minutes", max_guard, max_minutes);

    return max_guard;
}

fn find_minute_where_max_guard_sleeps_most(max_guard: i32, guard_datas: &HashMap<GuardDayDataIndex, BitVec<u32>>) -> i32{
    let mut minute_map = HashMap::new();

    for (idx, data) in guard_datas {
        if idx.guard_id != max_guard {
            continue;
        }

        for minute in 0..59 {
            if data.get(minute).unwrap() {
                let sleep_data = minute_map.entry(minute).or_insert(0);
                *sleep_data = *sleep_data + 1;
            }
        }
    }

    let mut max_minute = 0;
    let mut max_sleep = -1;
    for (minute, sleep) in minute_map {
        if sleep > max_sleep {
            max_minute = minute;
            max_sleep = sleep;
        }
    }

    println!("guard {} slept at the most in minute {} for a total of {}", max_guard, max_minute, max_sleep);

    return max_minute as i32;

}

fn new_guard_data() -> BitVec {
    return BitVec::from_elem(60, false);
}

