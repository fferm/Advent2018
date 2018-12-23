extern crate regex;

use std::fs;
use std::fmt;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let small_input = true;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let nanobots = read_inputs(filename);

    let mut num_bots_in_range = HashMap::new();
    for bot in nanobots.clone() {
        println!("Analyzing bot at: {:?} with range: {}", bot.pos, bot.range);

        for point in bot.pos.points_within_distance(bot.range) {
            let mut entry = num_bots_in_range.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    println!("{:?}", num_bots_in_range);


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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
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

    fn points_within_distance(&self, distance: isize) -> Vec<Coord> {
        let mut ret = Vec::new();

        ret.push(self.clone());

        for current_distance in 0 .. distance + 1 {
            let points_at_distance = self.points_at_distance(current_distance);

            for p in points_at_distance {
                ret.push(p);
            }
        }

        return ret;

    }

    fn points_at_distance(&self, dist: isize) -> HashSet<Coord> {
        let mut ret = HashSet::new();

        for dx in -dist..dist+1 {
            let y_dist = dist - dx.abs();

            for dy in -y_dist .. y_dist + 1 {
                let dz = y_dist - dy.abs();

                ret.insert(Coord::new(self.x + dx, self.y + dy, self.z + dz));
                ret.insert(Coord::new(self.x + dx, self.y + dy, self.z - dz));
                ret.insert(Coord::new(self.x + dx, self.y - dy, self.z + dz));
                ret.insert(Coord::new(self.x + dx, self.y - dy, self.z - dz));
                ret.insert(Coord::new(self.x - dx, self.y + dy, self.z + dz));
                ret.insert(Coord::new(self.x - dx, self.y + dy, self.z - dz));
                ret.insert(Coord::new(self.x - dx, self.y - dy, self.z + dz));
                ret.insert(Coord::new(self.x - dx, self.y - dy, self.z - dz));
            }
        }
        return ret;
    }


}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{},{})", self.x, self.y, self.z);
    }
}


