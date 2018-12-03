use std::fs;

#[derive(Debug)]
struct Claim {
    id: String,
    top_left: Coord,
    size: Coord
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32
}

fn main() {
    let file_contents = fs::read_to_string("input_small.txt").expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let c = Claim{ id: String::from("hej"), top_left: Coord{x:1, y:2}, size: Coord{x: 2, y:3}};

    let s = Coord {x: 1, y: 2};

    println!("{:?}", c);
}
