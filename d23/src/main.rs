extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let small_input = false;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let nanobots = read_inputs(filename);

    let mut largest_range = 0;
    let mut largest_bot: Nanobot = Nanobot{pos: Coord::new(0, 0, 0), range: 0};
    for bot in nanobots.clone() {
        if bot.range > largest_range {
            largest_range = bot.range;
            largest_bot = bot.clone();
        }
    }

    println!("Largest range is from bot: {:?}", largest_bot);

    let mut count = 0;
    for bot in nanobots {
        if largest_bot.other_is_in_range_of(&bot) {
            count += 1;
        }
    }

    println!("{} bots are in range", count);
}

fn read_inputs<'a>(filename: &str) -> HashSet<Nanobot> {
    let mut ret = HashSet::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
    let lines: Vec<&str> = file_contents.split("\n").collect();

    for line in lines {
        let regex = "pos=<(-?\\d+),(-?\\d+),(-?\\d+)>, r=(\\d+)";
        let cap = Regex::new(regex).unwrap().captures_iter(line).next().expect("Error in regex");

        let pos = Coord::new(cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap());
        let nanobot = Nanobot{pos, range: cap[4].parse().unwrap()};
        ret.insert(nanobot);
    }

    return ret;
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Nanobot {
    pos: Coord,
    range: isize
}

impl Nanobot {
    fn other_is_in_range_of(&self, other: &Nanobot) -> bool {
        let distance = self.pos.manhattan_distance_from(other.pos);

        return distance <= self.range;
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Coord {
    x: isize,
    y: isize,
    z: isize
}

impl Coord {
//    fn coords_in_range(&self) -> Vec<Coord> {
//        return vec![
//            Coord{x: self.x, y: self.y - 1},    // ordering is important (reading order)
//            Coord{x: self.x - 1, y: self.y},
//            Coord{x: self.x + 1, y: self.y},
//            Coord{x: self.x, y: self.y + 1}
//        ];
//    }

    fn new(x: isize, y: isize, z: isize) -> Coord {
        return Coord{x, y, z}
    }

//    fn mv(&self, dir: Direction) -> Coord {
//        match dir {
//            Direction::Left => Coord{x: self.x - 1, y: self.y},
//            Direction::Right => Coord{x: self.x + 1, y: self.y},
//            Direction::Up => Coord{x: self.x, y: self.y - 1},
//            Direction::Down => Coord{x: self.x, y: self.y + 1}
//        }
//    }

    fn manhattan_distance_from(&self, other: Coord) -> isize {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        let z_dist = (self.z - other.z).abs();

        return x_dist + y_dist + z_dist;
    }
}

