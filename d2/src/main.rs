use std::fs;
use std::collections::HashMap;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Error in reading file");

    let spl = file_contents.split("\n");

    let mut two_same = 0;
    let mut three_same = 0;

    for word in spl {
        let chars = word.chars();

        let mut chars_found = HashMap::new();

        for c in chars {
            let count = chars_found.entry(c).or_insert(0);
            *count += 1;
//            println!("c {}    count {}", c, count);
        }

        for (_, value) in &chars_found {
            if *value == 2 {
                two_same = two_same + 1;
                break;
            }
        }

        for (_, value) in &chars_found {
            if *value == 3 {
                three_same = three_same+ 1;
                break;
            }
        }

        println!("{}", word);
    }

    println!("two_same: {}      three_same {}       hash {}", two_same, three_same, two_same * three_same);
}
