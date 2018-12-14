fn main() {
    let small_input = false;
    let mut recepies = vec![3,7];

    let num_recepies: usize;
    if small_input {
        num_recepies = 2018;
    } else {
        num_recepies = 846021;
    }

    let mut current_idx1: usize =  0;
    let mut current_idx2: usize = 1;

    print_simulation(&recepies, current_idx1, current_idx2);

    while recepies.len() < num_recepies + 10 {
        run_simulation(&mut recepies, &mut current_idx1, &mut current_idx2);

        if num_recepies < 50 {
            print_simulation(&recepies, current_idx1, current_idx2);
        } else if recepies.len() % 1000 == 0 {
            println!("number of recepies: {},   target: {}", recepies.len(), num_recepies);
        }
    }


    print!("Last 10: {:?}", &recepies[num_recepies .. num_recepies + 10]);
}

fn run_simulation(recepies: &mut Vec<isize>, current_idx1: &mut usize, current_idx2: &mut usize) {
    let num1: isize;
    let num2: isize;
    {
        num1 = *recepies.get(*current_idx1).unwrap();
        num2 = *recepies.get(*current_idx2).unwrap();
    }

    let sum = num1 + num2;

    if sum >= 10 {
        recepies.push(1);
    }
    let ones = sum % 10;
    recepies.push(ones);

    *current_idx1 = (1 + *current_idx1 + num1 as usize) % recepies.len();
    *current_idx2 = (1 + *current_idx2 + num2 as usize) % recepies.len();
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
