extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Input {
    id: char,
    c: Coord,
    is_infinite: bool = false,
    score: i32 = 0
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct BoardPointData {
    has_closest_input: bool,
    closest_input: char,
    closest_input_distance: i32
}

fn main() {
    let filename = "input_small.txt";
    let inputs = read_inputs(filename);
    println!("Inputs");
    for c in &inputs {
        println!("{:?}", c);
    }

    let mut max_x: i32 = -1000;
    let mut max_y: i32 = -1000;
    let mut min_x: i32 = 1000;
    let mut min_y: i32 = 1000;

    for c in &inputs {
        if c.c.x < min_x {
            min_x = c.c.x;
        }
        if c.c.y < min_y {
            min_y = c.c.y;
        }
        if c.c.x > max_x {
            max_x = c.c.x;
        }
        if c.c.y > max_y {
            max_y = c.c.y;
        }
    }

    println!("max_x: {}  max_y: {}  min_x: {}  min_y: {}", max_x, max_y, min_x, min_y);

    let mut board: HashMap<Coord, BoardPointData> = HashMap::new();

    for input in &inputs {
        board.insert(input.c.clone(), BoardPointData{has_closest_input: true, closest_input: input.id, closest_input_distance: 0});

        let mut current_distance = 0;
        let mut did_insert = true;

        while did_insert {
            did_insert = false;
            current_distance = current_distance + 1;

            let points_with_distance = points_with_distance(&input.c, current_distance);


            for current_point in points_with_distance {
                if current_point.x < min_x || current_point.x > max_x || current_point.y < min_y || current_point.y > max_y {
                    continue;
                }
                let mut data_to_insert: Option<BoardPointData>;

                match &board.entry(current_point.clone()) {
                    Occupied(previous_entry) => {
                        let previous = previous_entry.get();

                        if previous.closest_input_distance < current_distance {
                            // Do nothing
                            data_to_insert = None;
                        } else if previous.closest_input_distance == current_distance {
                            // Equal distance
                            data_to_insert = Some(BoardPointData{has_closest_input: false, closest_input: '.', closest_input_distance: current_distance});

                        } else {
                            // Current one is smaller, overwrite
                            data_to_insert = Some(BoardPointData{has_closest_input: true, closest_input: input.id, closest_input_distance: current_distance});
                        }
                    },
                    Vacant(_) => {
                        data_to_insert = Some(BoardPointData{has_closest_input: true, closest_input: input.id, closest_input_distance: current_distance});
                    }
                }

                if data_to_insert.is_some() {
//                    println!("Inserting {:?} on {:?}", data_to_insert.clone().unwrap(), current_point);
                    &board.insert(current_point.clone(), data_to_insert.unwrap());
                    did_insert = true;
                }
            }
        }
    }

    if filename == "input_small.txt" {
        print_board(&board, max_x, max_y, min_x, min_y);
    }


//    let mut ids_and_scores
    for j in min_y..max_y+1 {
        for i in min_x..max_x+1 {
            let current_coord = Coord{x: i, y: j};


        }
    }
}

fn points_with_distance(c: &Coord, dist: i32) -> Vec<Coord> {
    let mut ret = Vec::new();

    for i in -dist..dist+1 {
        let x = c.x + i;

        let mut y1: i32 = 0;
        let mut y2: i32 = 0;

        if i < 0 {
            y1 = c.y + dist + i;
            y2 = c.y - dist - i;
        } else {
            y1 = c.y + dist - i;
            y2 = c.y - dist + i;
        }

//        println!("i: {:2}  c.x: {:2} c.y: {:2}   dist: {:2}   x: {:2},  y1: {:2}, y2: {:2}", i, c.x, c.y, dist, x, y1, y2);
        ret.push(Coord{x: x, y: y1});

        if y1 != y2 {
            ret.push(Coord{x: x, y: y2});
        }
    }

//    println!();


    return ret;
}

fn print_board(board: &HashMap<Coord, BoardPointData>, max_x: i32, max_y: i32, min_x: i32, min_y: i32) {
    for j in min_y..max_y+1 {
        for i in min_x..max_x+1 {
            let current_coord = Coord{x: i, y: j};

            let data = board.get(&current_coord);

            match data {
                Some(d) => {
                    if d.closest_input_distance == 0 {
                        print!("{} ", d.closest_input);
                    } else if !d.has_closest_input {
                        print!(". ");
                    } else {
                        print!("{} ", d.closest_input.to_lowercase());
                    }
                },
                None => {
                    print!("- ");
                }
            }
        }

        println!();
    }
}

fn read_inputs(filename: &str) -> Vec<Input> {
    let mut ret = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut id = 'A';
    for line in lines {
        let re = Regex::new("(\\d+), (\\d+)").unwrap();

        let cap = re.captures_iter(line).next().expect("Error in capturing regex");

        ret.push(Input{id: id, c: Coord{x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap()}});

        id = std::char::from_u32(id as u32 + 1).unwrap();
    }

    return ret;
}
