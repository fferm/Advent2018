extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::cell::Cell;
use std::fmt;

fn main() {
    let small_input = true;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut sim = read_inputs(filename);
    println!("{:?}", sim);
}

fn read_inputs(filename: &str) -> Sim {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut clay = HashSet::new();
    let settled_water = HashSet::new();
    let flowing_water = HashSet::new();
    let mut left = std::isize::MAX;
    let mut right = std::isize::MIN;
    let mut bottom = std::isize::MIN;

    for current_line in lines {
        let instruction_regex = "(x|y)=(\\d+), (x|y)=(\\d+)..(\\d+)";

        let cap = Regex::new(&instruction_regex).unwrap().captures_iter(current_line).next().expect("Error in capturing instruction regex");

        let first_var = &cap[1];
        let first_value = cap[2].parse().unwrap();

        let _second_var = &cap[3];
        let second_min_value: isize = cap[4].parse().unwrap();
        let second_max_value: isize = cap[5].parse().unwrap();

        for second_val in second_min_value .. second_max_value + 1 {
            if first_var == "x" {
                clay.insert(Coord{x: first_value, y: second_val});
            } else {
                clay.insert(Coord{x: second_val, y: first_value});
            }
        }

        for clay_coord in &clay {
            if clay_coord.x < left {
                left = clay_coord.x;
            }
            if clay_coord.x > right {
                right = clay_coord.x
            }
            if clay_coord.y > bottom {
                bottom = clay_coord.y
            }
        }
    }

    left -= 1;
    right += 1;

    return Sim{source: Coord{x:500, y:0}, clay, settled_water, flowing_water, left, right, bottom};
}

struct Sim {
    source: Coord,
    clay: HashSet<Coord>,
    settled_water: HashSet<Coord>,
    flowing_water: HashSet<Coord>,
    left: isize,
    right: isize,
    bottom: isize
}

impl fmt::Debug for Sim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0 .. self.bottom + 1 {
            for x in self.left .. self.right + 1 {
                let coord = Coord{x, y};
                let mut ch = ".";
                if self.clay.contains(&coord) {
                    ch = "#";
                } else if self.flowing_water.contains(&coord) {
                    ch = "|";
                } else if self.settled_water.contains(&coord) {
                    ch = "~";
                } else if coord == self.source {
                    ch = "+";
                }

                ret.push_str(&ch);
            }
            ret.push_str(&"\n");
        }

        return write!(f, "{}", ret);
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Coord {
    y: isize,
    x: isize
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{})", self.x, self.y);
    }
}

