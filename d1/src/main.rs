use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Error in reading file");

    let mut spl = file_contents.split("\n");

    let mut i = 0;
    let mut result = 0;
    for s in spl {
        let int_value: i32 = s.parse().unwrap();
        result = result + int_value;
        println!("i: {}    s: {}     int_value: {}    result: {}", i, s, int_value, result);
        i = i + 1;
    }
}
