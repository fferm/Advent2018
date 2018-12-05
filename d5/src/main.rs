extern crate regex;

use std::fs;

fn main() {
    let mut working_string = read_inputs("input.txt");

    println!("{}  {}", working_string.len(), working_string);

    let mut did_change = true;
    while did_change {
        println!();
        did_change = false;

        for i in 0..working_string.len() - 1 {
            let non_mutable = working_string.clone();

            let char1 = non_mutable.get(i..i + 1).unwrap();
            let char2 = non_mutable.get(i + 1..i + 2).unwrap();

//            println!("{} and {}", char1, char2);

            if char1 != char2 && char1.to_uppercase() == char2.to_uppercase() {
                let first_part = non_mutable.get(..i).unwrap();
                let second_part = non_mutable.get(i + 2..).unwrap();

                working_string = format!("{}{}", first_part, second_part);

                println!("{}  {}", working_string.len(), working_string);
                did_change = true;
                break;
            }
        }
    }


}

fn read_inputs(filename: &str) -> String {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    return file_contents;
}
