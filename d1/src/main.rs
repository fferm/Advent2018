use std::fs;

fn main() {

    let mut values_found = Vec::new();
    values_found.push(0);

    let mut i = 0;
    let mut result = 0;

    let mut solution = 0;
    let mut solution_found = false;


    let file_contents = fs::read_to_string("input.txt").expect("Error in reading file");
    loop {

        let spl = file_contents.split("\n");

        for s in spl {
            let int_value: i32 = s.parse().unwrap();

            result = result + int_value;
//            println!("int_value: {}   Result: {}", int_value, result);

            if value_in_vector(&values_found, &result) {
                println!("Solution found {}", result);
                solution = result;
                solution_found = true;
            } else {
                values_found.push(result);
//                println!("Pushing {}, size is {}", result, values_found.iter().count())
            }

          if solution_found {
              println!("Breaking loop");
            break;
          }
//        println!("i: {}    s: {}     int_value: {}    result: {}", i, s, int_value, result);
            i = i + 1;
        }

        if solution_found {
            break;
        }
        println!("Looping   Size is {}", values_found.iter().count());
    }
    println!("Solution: {}", solution);


}

fn value_in_vector(v: &Vec<i32>, val: &i32) -> bool {
    for i in v {
//        println!("Checking {} against {}", i, val);
        if i == val {
            return true;
        }
    }
    return false;
}
