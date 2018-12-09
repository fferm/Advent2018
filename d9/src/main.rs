
struct CircleElement {
    value: i32,
    left: &'static CircleElement,
    right: &'static CircleElement
}

impl CircleElement {
    fn go_right(&self) -> &CircleElement {
        return self.right;
    }

    fn go_left(&self) -> &CircleElement {
        return self.left;
    }

    fn insert(&self, value: i32) -> &CircleElement {
        let new_element = CicleElement{value: value, left: self.left, right: self.right};
        return &new_element;
    }

    fn remove(&self) -> &CircleElement {
        let ret = self.right;

    }
}



fn main() {
    let num_players = 424;
    let max_points = 71482 * 100;

//    let num_players = 13;
//    let max_points = 7999;

    let mut circle = Vec::new();
    circle.push(0);

    if max_points < 100 {
        print_current_circle(0, &circle, 0);
    }

    let mut current_marble = 0;
    let mut next_marble = 1;
    let mut current_player = 1;
    let mut current_marble_idx: i64 = 0;

    let mut scores = Vec::new();
    for _i in 0..num_players + 1 {
        scores.push(0);
    }

    while next_marble <= max_points {
        let ulen = circle.len();

        if next_marble % 23 == 0 {

            let remove_idx = wrap_idx(current_marble_idx - 7, ulen);
            let current_idx = wrap_idx(remove_idx as i64 + 1, ulen);

            current_marble = circle.get(current_idx).unwrap().clone();
            let removed_marble = circle.get(remove_idx).unwrap().clone();

            add_score_to_player(&mut scores, removed_marble, current_player);
            add_score_to_player(&mut scores, next_marble, current_player);

            &circle.remove(remove_idx as usize);

            current_marble_idx = remove_idx as i64;

        } else {
            let insert_idx = wrap_idx(current_marble_idx as i64 + 2, ulen);
            &circle.insert(insert_idx, next_marble);

            current_marble = next_marble;
            current_marble_idx = insert_idx as i64;
        }

//        print_current_circle(current_player, &circle, current_marble);


        next_marble = next_marble + 1;
        current_player = (current_player % num_players) + 1;

        if next_marble % 10000 == 0 {
            println!("Playing...  next_marble: {}   target: {},    size: {}", next_marble, max_points, &circle.len());
        }
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

fn wrap_idx(input: i64, size: usize) -> usize {
    let mut iret = input;

    let ilen = size as i64;

    while iret < 0 {
        iret = iret + ilen;
    }

    while iret > (ilen - 1) {
        iret = iret - ilen;
    }

    return iret as usize;
}


fn add_score_to_player(scores: &mut Vec<i64>, score: i64, player: i64) {
    let prev_score = scores.get(player as usize).unwrap();
    let new_score = *prev_score + score;

    scores.remove(player as usize);
    scores.insert(player as usize, new_score);
}

fn print_current_circle(current_player: i64, circle: &Vec<i64>, current_marble: i64) {
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
