fn main() {
    let small_input = true;
    let mut recepies: Vec<isize>;

    if small_input {
        recepies = vec![3,7]
    } else {
        recepies = vec![8, 4, 6, 0, 2, 1];
    }

    let mut current_idx1: usize =  0;
    let mut current_idx2: usize = 1;

    print_simulation(&recepies, current_idx1, current_idx2);
    run_simulation(&mut recepies, &mut current_idx1, &mut current_idx2);
}

fn run_simulation(recepies: &mut Vec<isize>, current_idx1: &mut usize, current_idx2: &mut usize) {
    let num1 = recepies.get(*current_idx1).unwrap();
    let num2 = recepies.get(*current_idx2).unwrap();

    let sum = num1 + num2;

    println!("{}", sum);
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
