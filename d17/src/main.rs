extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let small_input = false;
    let filename;

    if small_input {
//        filename = "input_small.txt";
        filename="input_test.txt";
    } else {
        filename = "input.txt";
    }

    let mut contents = HashMap::new();
    let mut analysis_flow = Vec::new();

    let mut sim = read_inputs(filename, &mut contents, &mut analysis_flow);
    if small_input {
        println!("{:?}", sim);
    }

    let mut step = 0;
    while sim.run() {
        println!("Step: {}", step);
        if small_input {
//            println!("{:?}", sim);
        }
        step += 1;
    }

    println!("{:?}", sim);
    println!("A total of {} units of water in {} steps", sim.num_water(), step);

}

fn read_inputs<'a>(filename: &str, contents: &'a mut HashMap<Coord, Contents>, analysis_flow: &'a mut Vec<Coord>) -> Sim<'a> {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let source = Coord { x: 500, y: 0 };

    analysis_flow.push(source.clone());

    let mut left = std::isize::MAX;
    let mut right = std::isize::MIN;
    let mut bottom = std::isize::MIN;
    let mut top_clay = std::isize::MAX;

    for current_line in lines {
        let instruction_regex = "(x|y)=(\\d+), (x|y)=(\\d+)..(\\d+)";

        let cap = Regex::new(&instruction_regex).unwrap().captures_iter(current_line).next().expect("Error in capturing instruction regex");

        let first_var = &cap[1];
        let first_value = cap[2].parse().unwrap();

        let _second_var = &cap[3];
        let second_min_value: isize = cap[4].parse().unwrap();
        let second_max_value: isize = cap[5].parse().unwrap();

        for second_val in second_min_value..second_max_value + 1 {
            let coord: Coord;
            if first_var == "x" {
                coord = Coord { x: first_value, y: second_val };
            } else {
                coord = Coord { x: second_val, y: first_value };
            }

            if coord.x < left {
                left = coord.x;
            }
            if coord.x > right {
                right = coord.x
            }
            if coord.y > bottom {
                bottom = coord.y
            }
            if coord.y < top_clay {
                top_clay = coord.y
            }

            contents.insert(coord, Contents::Clay);
        }
    }

    left -= 1;
    right += 1;

    return Sim { source, contents, analysis_flow, left, right, bottom, top_clay };
}

struct Sim<'a> {
    source: Coord,
    contents: &'a mut HashMap<Coord, Contents>,
    analysis_flow: &'a mut Vec<Coord>,
    left: isize,
    right: isize,
    bottom: isize,
    top_clay: isize,
}

impl<'a> Sim<'a> {
    fn run(&mut self) -> bool {
        let can_flow_info = self.can_flow(true);
        if can_flow_info.is_some() {
            self.flow_in_direction(can_flow_info.unwrap());
            return true;
        }

        // If we get here, it was impossible to flow
        if self.are_we_settled() {
            self.settle();
            return true;
        } else {
            return self.backtrack();
        }
    }

    fn can_flow(&self, down_ok: bool) -> Option<Direction> {
        if down_ok {
            let below = self.get_flow_head().mv(Direction::Down);
            if self.can_flow_to(&below) {
                return Some(Direction::Down);
            }
        }

        if self.on_solid_ground() {
            if self.can_flow_to(&self.get_flow_head().mv(Direction::Left)) {
                return Some(Direction::Left);
            }
            if self.can_flow_to(&self.get_flow_head().mv(Direction::Right)) {
                return Some(Direction::Right);
            }
        }

        return None;
    }

    fn flow_in_direction(&mut self, dir: Direction) {
        let c = self.get_flow_head().mv(dir);

        self.analysis_flow.push(c.clone());
        self.contents.insert(c, Contents::Flowing);
    }

    fn can_flow_to(&self, target: &Coord) -> bool {
        if !self.has_target_inside(target) {
            return false;
        }
        return self.get_contents(target) == Contents::Sand;
    }

    fn on_solid_ground(&self) -> bool {
        let contents_below = self.get_contents(&self.get_flow_head().mv(Direction::Down));

        return contents_below == Contents::Clay || contents_below == Contents::Settled;
    }

    fn are_we_settled(&self) -> bool {
        let settle_limit_left_contents = self.find_settle_limit_in_direction(Direction::Left).0;
        let settle_limit_right_contents = self.find_settle_limit_in_direction(Direction::Right).0;

        return settle_limit_left_contents == Contents::Clay && settle_limit_right_contents == Contents::Clay;
    }

    fn settle(&mut self) {
        let limit_left = self.find_settle_limit_in_direction(Direction::Left).1;
        let limit_right = self.find_settle_limit_in_direction(Direction::Right).1;

        for x in limit_left.x .. limit_right.x + 1 {
            let coord = Coord{x, y: limit_left.y};

            self.contents.insert(coord, Contents::Settled);
        }

        while self.get_contents(&self.get_flow_head()) == Contents::Settled {
            self.analysis_flow.pop();
        }
    }

    fn find_settle_limit_in_direction(&self, direction: Direction) -> (Contents, Coord) {
        let mut current = self.get_flow_head().clone();

        loop {
            let further = current.mv(direction);
            let contents_further = self.get_contents(&further);

            if contents_further == Contents::Clay {
                return (Contents::Clay, current);
            }

            if contents_further == Contents::Sand {
                return (Contents::Sand, current);
            }

            current = further.clone();
        }
    }

    fn backtrack(&mut self) -> bool {   // Flag that shows if we can continue
        loop {
            if self.analysis_flow.len() == 0 {
                return false;
            }
            if self.can_flow(false).is_some() {
                return true;
            }

            self.analysis_flow.pop();
        }
    }

    fn has_target_inside(&self, target: &Coord) -> bool {
        return target.x >= self.left && target.x <= self.right && target.y <= self.bottom;
    }

    fn num_water(&self) -> usize {
        let mut ret = 0;

        for y in self.top_clay..self.bottom + 1 {
            for x in self.left .. self.right + 1 {
                let contents = self.get_contents(&Coord{x, y});
                if contents == Contents::Settled || contents == Contents::Flowing {
                    ret += 1;
                }
            }
        }
        return ret;
    }

    fn get_contents(&self, target: &Coord) -> Contents {
        return *self.contents.get(target).unwrap_or(&Contents::Sand);
    }

    fn is_flow_head(&self, target: &Coord) -> bool {
        return self.get_flow_head() == *target;
    }

    fn get_flow_head(&self) -> Coord {
        return self.analysis_flow.get(self.analysis_flow.len() - 1).unwrap().clone();
    }
}

impl<'a> fmt::Debug for Sim<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0..self.bottom + 1 {
            for x in self.left..self.right + 1 {
                let coord = Coord { x, y };
                let mut ch = ".";

                let contents = self.get_contents(&coord);

                if contents == Contents::Clay {
                    ch = "#";
//                } if self.is_flow_head(&coord) {
//                    ch = "0";
                } else if contents == Contents::Flowing {
                    ch = "|";
                } else if contents == Contents::Settled {
                    ch = "~";
                } else if coord == self.source {
                    ch = "+";
                }

                ret.push_str(&ch);
            }
            ret.push_str(&"\n");
        }

        ret.push_str(&"\n");

        return write!(f, "{}", ret);
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Contents {
    Clay,
    Flowing,
    Settled,
    Sand
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



