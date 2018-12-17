extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let small_input = true;
    let filename;

    if small_input {
//        filename = "input_small.txt";
        filename="input_test.txt";
    } else {
        filename = "input.txt";
    }

    let mut flowing_water = Vec::new();
    let mut flow_heads = Vec::new();

    let mut sim = read_inputs(filename, &mut flowing_water, &mut flow_heads);
    if small_input {
        println!("{:?}", sim);
    }

    let mut step = 0;
    while sim.run() {
        println!("Step: {}", step);
        if small_input {
            println!("{:?}", sim);
        }
        step += 1;
    }


    println!("{:?}", sim);
    println!("A total of {} units of water in {} steps", sim.num_water(), step);

}

fn read_inputs<'a>(filename: &str, flowing_water: &'a mut Vec<Coord>, flow_heads: &'a mut Vec<Coord>) -> Sim<'a> {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut clay = HashSet::new();
    let settled_water = HashSet::new();
    let source = Coord { x: 500, y: 0 };
    flow_heads.push(source.clone());
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

        for second_val in second_min_value..second_max_value + 1 {
            if first_var == "x" {
                clay.insert(Coord { x: first_value, y: second_val });
            } else {
                clay.insert(Coord { x: second_val, y: first_value });
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

    return Sim { source, clay, settled_water, flowing_water, flow_heads, left, right, bottom };
}

struct Sim<'a> {
    source: Coord,
    clay: HashSet<Coord>,
    settled_water: HashSet<Coord>,
    flowing_water: &'a mut Vec<Coord>,
    flow_heads: &'a mut Vec<Coord>,
    left: isize,
    right: isize,
    bottom: isize,
}

impl<'a> Sim<'a> {
    fn run(&mut self) -> bool {
        if self.flow_heads.len() > 0 {
            let flow_head = self.flow_heads.pop().unwrap();

            let down_from_flow_head = flow_head.mv(Direction::Down);
            let left_from_flow_head = flow_head.mv(Direction::Left);
            let right_from_flow_head = flow_head.mv(Direction::Right);

            // Flow if possible
            if self.can_flow_to(&down_from_flow_head) {
                if down_from_flow_head.y <= self.bottom {
                    self.flow_heads.push(down_from_flow_head);
                    self.flowing_water.push(down_from_flow_head);
                }
            } else {
                if self.can_flow_to(&left_from_flow_head) {
                    self.flowing_water.push(left_from_flow_head);
                    self.flow_heads.push(left_from_flow_head);
                }
                if self.can_flow_to(&right_from_flow_head) {
                    self.flowing_water.push(right_from_flow_head);
                    self.flow_heads.push(right_from_flow_head);
                }
            }
        } else {
            let flow_size = self.flowing_water.len();
            let latest_flow = self.flowing_water.get(flow_size - 1).unwrap().clone();
            let second_latest = self.flowing_water.get(flow_size - 2).unwrap().clone();

            if latest_flow.y == second_latest.y {
                // Handle blocked water
                let mut left: Coord = latest_flow.clone();
                let mut right: Coord = latest_flow.clone();

                while !self.is_blocked_at(&left.mv(Direction::Left)) {
                    left = left.mv(Direction::Left)
                }

                while !self.is_blocked_at(&right.mv(Direction::Right)) {
                    right = right.mv(Direction::Right)
                }

                for x in left.x..right.x + 1 {
                    let coord = Coord { x: x, y: latest_flow.y };
                    self.settled_water.insert(coord);
                    self.flowing_water.pop();
                }

                let new_flow_head = self.flowing_water.get(self.flowing_water.len() - 1).unwrap();
                self.flow_heads.push(new_flow_head.clone());
            } else {
                return false;
            }
        }

        return true;
    }

    fn can_flow_to(&self, target: &Coord) -> bool {
        return !self.is_blocked_at(target) && !self.flowing_water.contains(target);
    }

    fn is_blocked_at(&self, target: &Coord) -> bool {
        return self.clay.contains(target) || self.settled_water.contains(target);
    }

    fn num_water(&self) -> usize {
        return self.flowing_water.len() + self.settled_water.len();
    }
}

impl<'a> fmt::Debug for Sim<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0..self.bottom + 1 {
            for x in self.left..self.right + 1 {
                let coord = Coord { x, y };
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
        ret.push_str(&"Flow heads: ");
        for flow_head in self.flow_heads.clone() {
            ret.push_str(&format!("{:?}", flow_head));
            ret.push_str(&", ");
        }
        ret.push_str(&"\n");

        return write!(f, "{}", ret);
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    y: isize,
    x: isize,
}

impl Coord {
    fn mv(&self, dir: Direction) -> Coord {
        match dir {
            Direction::Left => Coord { x: self.x - 1, y: self.y },
            Direction::Right => Coord { x: self.x + 1, y: self.y },
            Direction::Up => Coord { x: self.x, y: self.y - 1 },
            Direction::Down => Coord { x: self.x, y: self.y + 1 }
        }
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{})", self.x, self.y);
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}



