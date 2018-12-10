extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord {
    x: i32,
    y: i32
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Point {
    pos: Coord,
    d: Coord
}

fn main() {
    let small_input = false;
    let filename: &str;
    let x_limit = 150;
    let y_limit = 50;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut board: Vec<Point> = read_inputs(filename);

    let mut cont = true;
    let mut shrinking = true;

    let mut prev_x_dist = 1000000;
    let mut prev_y_dist = 1000000;
    let mut i = 0;
    while cont {
        let min_max = min_max(&board);  // (min_x, max_x, min_y, max_y)

        let x_dist = min_max.1 - min_max.0;
        let y_dist = min_max.3 - min_max.2;

        if x_dist <= x_limit && y_dist <= y_limit {
            println!("Num seconds: {}", i);
            print_board(&board, min_max);
            println!();
        }

        board = move_board(&board);

        if shrinking && x_dist > prev_x_dist && y_dist > prev_y_dist {
            shrinking = false;
        }

        prev_x_dist = x_dist;
        prev_y_dist = y_dist;

        i = i + 1;

        cont = shrinking || (x_dist < x_limit && y_dist < y_limit);

        if i %& 1000 == 0 {
            println!("i: {}, x_dist: {}, y_dist: {}", i, x_dist, y_dist);
        }
    }

//    println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_max.0, min_max.1, min_max.2, min_max.3);
}

fn read_inputs(filename: &str) -> Vec<Point> {
    let mut ret = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    for line in lines {
        let regex = "position=<\\s*([-]?\\d+),\\s*([-]?\\d+)> velocity=<\\s*([-]?\\d+),\\s*([-]?\\d+)>";
        let re = Regex::new(regex).unwrap();
        let cap = re.captures_iter(line).next().expect("Error in capturing regex");

        let current_pos = Coord{x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap()};
        let current_d = Coord{x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap()};

        let current_point = Point{pos: current_pos, d: current_d};

        ret.push(current_point);
    }

    return ret;
}

fn min_max(board: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut min_x = 10000;
    let mut min_y = 10000;

    let mut max_x = -10000;
    let mut max_y = -10000;

    for p in board {
        if p.pos.x < min_x {
            min_x = p.pos.x;
        }
        if p.pos.y < min_y {
            min_y = p.pos.y;
        }
        if p.pos.x > max_x {
            max_x = p.pos.x;
        }
        if p.pos.y > max_y {
            max_y = p.pos.y
        }
    }

    return (min_x, max_x, min_y, max_y);
}

fn print_board(board: &Vec<Point>, min_max: (i32, i32, i32, i32)) {
//    let min_max = min_max(&board);  // (min_x, max_x, min_y, max_y)

    println!("Board from x: [ {} .. {} ]   y: [ {} .. {} ]", min_max.0, min_max.1, min_max.2, min_max.3);
    println!();

    let mut map = HashMap::new();
    for p in board {
        map.insert(p.pos, true);
    }

    for y in min_max.2 .. min_max.3 + 1 {
        for x in min_max.0 .. min_max.1 + 1 {
            let coord = Coord{x: x, y: y};
            if map.contains_key(&coord) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn move_board(board: &Vec<Point>) -> Vec<Point> {
    let mut ret = Vec::new();

    for p in board {
        let new_coord = Coord{x: p.pos.x + p.d.x, y: p.pos.y + p.d.y};
        let new_p = Point{pos: new_coord, d: p.d.clone()};

        ret.push(new_p);
    }
    return ret;
}

