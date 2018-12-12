extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct RuleSelector {
    ll: bool,
    l: bool,
    c: bool,
    r: bool,
    rr: bool
}

fn main() {
    let small_input = false;
    let num_generations: isize = 50000000000;

    let filename: &str;
    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

/*    let input = read_inputs(filename);
    let initial_state = input.0;
    let rules = input.1;

    let minmax = min_max_coord(&initial_state);

    let mut current_gen = 0;
    let print_min = minmax.0 - 5;
    let print_max = minmax.1 + 100;
//    print_header(print_min, print_max);
//    print_state(&initial_state, current_gen, print_min, print_max);

    let mut working_state = initial_state;
    while current_gen < num_generations {
        current_gen += 1;

        working_state = run_generation(&working_state, &rules);

        if current_gen % 1000 == 0{
            println!("Current: {}   target: {}", current_gen, num_generations);
        }
//        print_state(&working_state, current_gen, print_min, print_max);
        println!("Current: {}   target: {}   answer: {}", current_gen, num_generations, sum_of_state(&working_state));
    }

    println!("Answer is {}", sum_of_state(&working_state));*/

    let mut answer: usize;
    let m: usize = 1113;
    let k: usize = 75;
    let x: usize = 50 * 1000 * 1000 * 1000;

    println!("k: {}  x: {}  m: {}   kx + m: {}", k, x, m, k * x + m);
    println!("max usize: {}", std::usize::MAX);
}

fn run_generation(working_state: &HashSet<isize>, rules: &HashMap<RuleSelector, bool>) -> HashSet<isize>{
    let mut ret = HashSet::new();

    let state_minmax = min_max_coord(working_state);

    for center_idx in (state_minmax.0 - 2) .. (state_minmax.1 + 3) {
        let ll = working_state.contains(&(center_idx - 2));
        let l = working_state.contains(&(center_idx - 1));
        let c = working_state.contains(&center_idx);
        let r = working_state.contains(&(center_idx + 1));
        let rr = working_state.contains(&(center_idx + 2));

        let selector = RuleSelector{ll:ll, l:l, c:c, r:r, rr: rr};

        if rules.contains_key(&selector) && *(rules.get(&selector).unwrap()) {
            ret.insert(center_idx);
        }
    }

    return ret;
}

fn read_inputs(filename: &str) -> (HashSet<isize>, HashMap<RuleSelector, bool>) {
    let mut initial_state = HashSet::new();
    let mut rules = HashMap::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let all_lines: Vec<&str> = file_contents.split("\n").collect();

    let initial_state_line = all_lines.get(0).unwrap();

    for i in 15 .. initial_state_line.len() {
        let c =  initial_state_line.get(i..i+1).unwrap();
        let b = tf(c);

        if b {
            initial_state.insert((i - 15) as isize);
        }
    }

    let rules_lines = &all_lines[2..];
    for line in rules_lines {
        let regex = "([.#])([.#])([.#])([.#])([.#]) => ([.#])";
        let re = Regex::new(regex).unwrap();
        let cap = re.captures_iter(line).next().expect("Error in capturing regex");

        let rule_selector = RuleSelector{ll: tf(&cap[1]), l: tf(&cap[2]), c: tf(&cap[3]), r: tf(&cap[4]), rr: tf(&cap[5])};
        let rule_value = tf(&cap[6]);

        rules.insert(rule_selector, rule_value);
    }

    return (initial_state, rules)
}

fn sum_of_state(state: &HashSet<isize>) -> isize {
    let mut ret = 0;
    for i in state {
        if state.contains(&i) {
            ret += i;
        }
    }
    return ret;
}

fn tf(input: &str) -> bool {
    return input == "#";
}

fn min_max_coord(state: &HashSet<isize>) -> (isize, isize) {
    let mut min = std::isize::MAX;
    let mut max = std::isize::MIN;

    for coord in state {
        if *coord < min {
            min = *coord;
        }
        if *coord > max {
            max = *coord;
        }
    }

    return (min, max);
}

fn print_header(min: isize, max: isize) {
    let spacer = "    ";
    print!("{}", spacer);


    for i in min..max + 1 {
        if i > 0 && i % 10 == 0 {
            print!("{}", i / 10);
        } else {
            print!(" ");
        }
    }
    println!("");

    print!("{}", spacer);
    for i in min..max + 1 {
        if i >= 0 {
            print!("{}", i % 10);
        } else {
            print!(" ");
        }
    }
    println!();
}

fn print_state(state: &HashSet<isize>, current_gen: isize, min: isize, max: isize) {
    print!("{:2}: ", current_gen);

    for i in min..max + 1 {
        if state.contains(&i) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();

}

