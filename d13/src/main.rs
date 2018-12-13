use std::fs;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum TurnDirection {
    Left,
    Straight,
    Right
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Cart {
    pos: Cell<Coord>,
    direction: Cell<Direction>,
    next_turn: Cell<TurnDirection>
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Track {
    pos: Coord,
    directions: Vec<Direction>
}

struct Sim {
    tracks: HashMap<Coord, Track>,
    carts: HashMap<Coord, Cart>
}

fn main() {
    let small_input = true;
    let num_generations: isize = 50000000000;

    let filename: &str;
    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    read_inputs(filename);
}


fn read_inputs(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut chars = HashMap::new();

    let mut y = 0;
    for line in lines {
        for x in 0..line.len() {
            let c = line.get(x..x+1).unwrap();
            if c != " " && c != "\r" && c!= "\n" {
                chars.insert(Coord{x: x as isize, y: y}, c);
            }
        }

        y += 1;
    }

    let mut tracks = HashMap::new();
    for (coord, c) in chars {
        if c == "-" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Left, Direction::Right]});
        } else if c == "|" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Up, Direction::Down]});
        } else if c == "+" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]});
        }
    }
    println!("{:?}", tracks);

}

