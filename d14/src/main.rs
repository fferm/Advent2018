fn main() {
    let mut recepies = vec![3,7];

  //  let search_pattern: [isize; 5] = [5, 9, 4, 1, 4];
    //let search_pattern: [isize; 6] = [ 9, 7, 8, 9, 6, 8];
    let search_pattern: [isize; 6] = [8, 4, 6, 0, 2, 1];

    let max_length = 1000000000;
    let mut current_idx1: usize =  0;
    let mut current_idx2: usize = 1;

    print_simulation(&recepies, current_idx1, current_idx2);

    let mut cont = true;
    while cont {
        run_simulation(&mut recepies, &mut current_idx1, &mut current_idx2);

        if recepies.len() < 20 {
            print_simulation(&recepies, current_idx1, current_idx2);
        } else if recepies.len() % 1000 == 0 {
            println!("number of recepies: {}", recepies.len());
        }

        if recepies.len() >= (search_pattern.len() + 1) {
            let match_tuple = does_it_match(&recepies, &search_pattern);

            if match_tuple.0 {
                println!("Match at: {}", match_tuple.1);
                cont = false;
            }
        }

        if recepies.len() > max_length {
            println!("För högt");
            cont = false;
        }
    }
}

fn does_it_match(recepies: &Vec<isize>, search_pattern: &[isize]) -> (bool, usize) {
    let part = &recepies[recepies.len() - search_pattern.len() .. recepies.len()];
    if *part == *search_pattern {
        return (true, recepies.len() - search_pattern.len());
    }

    let part_to_left = &recepies[recepies.len() - search_pattern.len() - 1 .. recepies.len() - 1];
    if *part_to_left == *search_pattern {
        return (true, recepies.len() - search_pattern.len() - 1);
    }

    return (false, 0);
}

fn run_simulation(recepies: &mut Vec<isize>, current_idx1: &mut usize, current_idx2: &mut usize) {
    let num1: isize;
    let num2: isize;
    {
        num1 = *recepies.get(*current_idx1).unwrap();
        num2 = *recepies.get(*current_idx2).unwrap();
    }

    let sum = num1 + num2;

/*    print!("num1: {}\tnum2: {}\tsum: {:2}\t\t", num1, num2, sum);*/

    if sum >= 10 {
        recepies.push(1);
    }
    let ones = sum % 10;
    recepies.push(ones);

    *current_idx1 = (1 + *current_idx1 + (num1 as usize)) % recepies.len();
    *current_idx2 = (1 + *current_idx2 + (num2 as usize)) % recepies.len();
}

fn print_simulation(recepies: &Vec<isize>, current_idx1: usize, current_idx2: usize) {
    for i  in 0..recepies.len() {
        if i == current_idx1 {
            print!("(");
        } else if i == current_idx2 {
            print!("[")
        } else {
            print!(" ");
        }

        print!("{}", recepies.get(i).unwrap());

        if i == current_idx1 {
            print!(")");
        } else if i == current_idx2 {
            print!("]")
        } else {
            print!(" ");
        }
    }
    println!();
}
