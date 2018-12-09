

fn main() {
    let num_players = 424;
    let max_points = 71482;

    let mut circle = Vec::new();
    circle.push(0);

    if max_points < 100 {
        print_current_circle(0, &circle, 0);
    }

    let mut current_marble = 0;
    let mut next_marble = 1;
    let mut current_player = 1;

    let mut scores = Vec::new();
    for _i in 0..num_players + 1 {
        scores.push(0);
    }

    while next_marble <= max_points {
        let current_marble_idx = *(&circle.iter().position( |i| *i == current_marble).unwrap());

        let ulen = circle.len();

        if next_marble % 23 == 0 {

            let remove_idx = wrap_idx(current_marble_idx as i32 - 7, ulen);
            let current_idx = wrap_idx(remove_idx as i32 + 1, ulen);

            current_marble = circle.get(current_idx).unwrap().clone();
            let removed_marble = circle.get(remove_idx).unwrap().clone();

            add_score_to_player(&mut scores, removed_marble, current_player);
            add_score_to_player(&mut scores, next_marble, current_player);

            &circle.remove(remove_idx as usize);

        } else {
            let insert_idx = wrap_idx(current_marble_idx as i32 + 2, ulen);
            &circle.insert(insert_idx, next_marble);

            current_marble = next_marble;
        }

//        print_current_circle(current_player, &circle, current_marble);


        next_marble = next_marble + 1;
        current_player = (current_player % num_players) + 1;
    }

    let mut max_score = 0;
    let mut max_player = 0;
    for idx in 1..num_players + 1 {
        let current_score = scores.get(idx as usize).unwrap().clone();
        if current_score > max_score {
            max_score = current_score;
            max_player = idx;
        }
    }
    println!("Player {} wins with {}", max_player, max_score);
}

fn wrap_idx(input: i32, size: usize) -> usize {
    let mut iret = input;

    let ilen = size as i32;

    while iret < 0 {
        iret = iret + ilen;
    }

    while iret > (ilen - 1) {
        iret = iret - ilen;
    }

    return iret as usize;
}


fn add_score_to_player(scores: &mut Vec<i32>, score: i32, player: i32) {
    let prev_score = scores.get(player as usize).unwrap();
    let new_score = *prev_score + score;

    scores.remove(player as usize);
    scores.insert(player as usize, new_score);
}

fn print_current_circle(current_player: i32, circle: &Vec<i32>, current_marble: i32) {
    print!("[{:2}] ", current_player);
    print!("[{:5}] ", current_marble);

    for marble in circle {
        if *marble == current_marble {
            print!("({:5})", marble);
        } else {
            print!(" {:5} ", marble);
        }
    }
    println!();
}
