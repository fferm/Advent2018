extern crate regex;

use std::fs;

fn main() {
    let input = read_inputs("input.txt");

    println!("input length: {}", input.len());

    //println!("{}  {}", input.len(), input);

    for c in get_alphabet() {
        process(&input, c);
    }
}

fn process(input: &String, lower_case: char) {
    let upper_case = lower_case.to_uppercase().next().unwrap();

    let mut working_string = remove_all_chars(input,lower_case);
    working_string = remove_all_chars(&working_string, upper_case);

    print!("Removing {}/{}   after_remove: {}", lower_case, upper_case, working_string.len());
    let best_string = react(&working_string);

    println!("     after_process: {}", best_string.len());
}

fn remove_all_chars(input: &String, c: char) -> String {
    return input.replace(c, "");
}

fn react(input: &String) -> String {
    let mut working_string = input.clone();

    let mut did_change = true;
    while did_change {
        let length_before = working_string.len();

        for lower_case in get_alphabet() {
            let upper_case = lower_case.to_uppercase().next().unwrap();

            working_string = working_string.replace(format!("{}{}", lower_case,upper_case).as_str(), "");
            working_string = working_string.replace(format!("{}{}", upper_case,lower_case).as_str(), "");
        }

        let length_after = working_string.len();
        did_change = length_after != length_before;

    return working_string
}

fn read_inputs(filename: &str) -> String {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    return file_contents;
}

fn get_alphabet() -> Vec<char> {
    (b'a'..b'z' + 1).filter_map(|c| Some(c as char)).collect::<Vec<_>>()
}
