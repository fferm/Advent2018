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
//        filename = "input_small.txt";
        filename="input_test.txt";
    } else {
        filename = "input.txt";
    }

    let mut flowing_water = Vec::new();

    let mut sim = read_inputs(filename, &mut flowing_water);
    if small_input {
        println!("{:?}", sim);
    }

    let mut step = 0;
    while sim.run() && step < 403 {
        println!("Step: {}", step);
        if small_input {
            println!("{:?}", sim);
        }
        step += 1;
    }


    println!("{:?}", sim);
    println!("A total of {} units of water in {} steps", sim.num_water(), step);

}

fn read_inputs<'a>(filename: &str, flowing_water: &'a mut Vec<Coord>) -> Sim<'a> {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut clay = HashSet::new();
    let settled_water = HashSet::new();
    let source = Coord { x: 500, y: 0 };

    let down_from_source = source.mv(Direction::Down);

    flowing_water.push(down_from_source.clone());

    let flow_head = Cell::new(down_from_source.clone());

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

    return Sim { source, clay, settled_water, flowing_water, flow_head, left, right, bottom };
}

struct Sim<'a> {
    source: Coord,
    clay: HashSet<Coord>,
    settled_water: HashSet<Coord>,
    flowing_water: &'a mut Vec<Coord>,
    flow_head: Cell<Coord>,
    left: isize,
    right: isize,
    bottom: isize,
}

impl<'a> Sim<'a> {
    fn run(&mut self) -> bool {
        let did_flow = self.flow_if_possible();
        if did_flow.is_some() {
            return true;
        }

        // If we get here, it was impossible to flow
        let settled_info = self.get_settled_info();
        println!("settled_info: {:?}", settled_info);
        if settled_info.is_some() {
            self.settle(settled_info);
            return true;
        } else {
            // Det är inte settling som orsakar att vi inte kan flöda, det är istället att vi nått botten
            loop {
                let current = self.flow_head.get();


                // TODO: Nu går flowing_water tillbaka efter att den nått botten
                // Lägg in HashMap med status per koordinat i Sim istället för clay och settled
                // Använd denna för flowing water också
                // låt flowing-water vektorn och flow-head (kanske kan slås ihop) till att bara peka ut aktuell ström
                // Gör display av flowing i vektorn och 'gammal' flowing olika, t ex 'F' för den ena
                // Förmodligen behöver hantering av settled göras om så att man inte blir settlad förrän bägge sidor är låsta (annars blir det fel i undre koppen som har olika höjd på sidorna

                if self.is_on_solid_ground(&current) && (self.can_flow_to(&current.mv(Direction::Left)) || self.can_flow_to(&current.mv(Direction::Right))) {
                    break;
                }
                self.flow_head.set(self.flowing_water.pop().unwrap().clone());
            }
        }

        return false;
    }

    fn flow_if_possible(&mut self) -> Option<Direction> {
        if self.flow_in_direction(Direction::Down) {
            return Some(Direction::Down);
        }
        if self.flow_in_direction(Direction::Left) {
            return Some(Direction::Left);
        }
        if self.flow_in_direction(Direction::Right) {
            return Some(Direction::Right);
        }

        return None;
    }

    fn flow_in_direction(&mut self, dir: Direction) -> bool {
        let c = self.flow_head.get().mv(dir);
        if self.can_flow_to(&c) {
            self.flow_head.set(c.clone());
            self.flowing_water.push(c.clone());
            return true;
        } else {
            return false;
        }
    }

    fn get_settled_info(&self) -> Option<(Coord, Coord, bool)> {  // Option<(Left, Right, other_side_reaches_sand)>
        if !self.is_on_solid_ground(&self.flow_head.get()) {
            return None;
        }

        let mut left: Coord = self.flow_head.get().clone();
        let mut right: Coord = self.flow_head.get().clone();

        let mut other_side_reaches_sand = false;

        loop {
            let further = left.mv(Direction::Left);
            let contents_of = self.get_contents(&further);

            match contents_of {
                Contents::Sand => {
                    other_side_reaches_sand = true;
                    break;
                },
                Contents::Flowing => left = further,
                _ => break
            }
        }

        loop {
            let further = right.mv(Direction::Right);
            let contents_of = self.get_contents(&further);

            match contents_of {
                Contents::Sand => {
                    other_side_reaches_sand = true;
                    break;
                },
                Contents::Flowing => right = further,
                _ => break
            }
        }

        return Some((left, right, other_side_reaches_sand));
    }

    fn settle(&mut self, settled_info_option: Option<(Coord, Coord, bool)>) {
        if settled_info_option.is_none() {
            return;
        }
        let settled_info = settled_info_option.unwrap();
        let mut start = self.flow_head.get();

        let mut current = start.clone();
        while current.is_inside_including_edge(&settled_info.0, &settled_info.1) {
            self.settled_water.insert(current);
            current = self.flowing_water.pop().unwrap();
        }

        self.flowing_water.push(current.clone());

        let other_side_reaches_sand = settled_info.2;

        if !other_side_reaches_sand {
            self.flow_head.set(current.clone());
        } else {
            self.flow_head.set(current.mv(Direction::Down));
        }
    }

    fn can_flow_to(&self, target: &Coord) -> bool {
        if !self.is_inside(target) {
            return false;
        }
        return !self.clay.contains(target) && !self.settled_water.contains(target) && !self.flowing_water.contains(target);
    }

    fn is_blocked_at(&self, target: &Coord) -> bool {
        return self.clay.contains(target) || self.settled_water.contains(target);
    }

    fn is_on_solid_ground(&self, target: &Coord) -> bool {
        let below = target.mv(Direction::Down);

        return self.clay.contains(&below) || self.settled_water.contains(&below);
    }

    fn is_inside(&self, target: &Coord) -> bool {
        return target.x >= self.left && target.x <= self.right && target.y <= self.bottom;
    }

    fn num_water(&self) -> usize {
        return self.flowing_water.len() + self.settled_water.len();
    }

    fn get_contents(&self, target: &Coord) -> Contents {
        if self.settled_water.contains(target) {
            return Contents::Settled;
        }

        if self.clay.contains(target) {
            return Contents::Clay;
        }

        if self.flowing_water.contains(target) {
            return Contents::Flowing;
        }

        return Contents::Sand;
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
                } if self.flow_head.get() == coord {
                    ch = "0";
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

    fn is_inside_including_edge(&self, top_left: &Coord, bottom_right: &Coord) -> bool {
        if self.x < top_left.x {
            return false;
        }
        if self.y < top_left.y {
            return false;
        }
        if self.x > bottom_right.x {
            return false;
        }
        if self.y > bottom_right.y {
            return false;
        }
        return true;
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



