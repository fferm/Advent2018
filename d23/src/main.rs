extern crate regex;

use std::fs;
use std::fmt;
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

    let mut vertexes = HashSet::new();
    for nanobot in nanobots.clone() {
        for vertex in nanobot.vertexes_from() {
            vertexes.insert(vertex);
        }
    }

//    let mut vertexes_in_range: HashMap<Coord, isize> = HashMap::new();
    let mut max_num = 0;
    let mut max_vertex = Coord::new(0, 0, 0);
    for vertex in vertexes {
        let num_nanobots_in_range = num_nanobots_in_range_from(&vertex, &nanobots);

        if num_nanobots_in_range > max_num {
            max_num = num_nanobots_in_range;
            max_vertex = vertex.clone();
            println!("Upping num to {}  from point {:?}", max_num, max_vertex);
        } else if num_nanobots_in_range == max_num {
            println!("Also {:?} has {}", vertex, num_nanobots_in_range);
        }

    }

    println!("Vertex: {:?}   num in range: {}     manhattan_distance: {}", max_vertex, max_num, max_vertex.manhattan_distance_from(Coord::new(0, 0, 0)));

    let mut point = max_vertex.clone();
    let mut range = max_num.clone();
    let mut distance = 0;

    let mut cont = true;
    while cont {
        cont = false;
        distance += 1;

        let points_at_distance = point.points_at_distance(distance);
        for other_point in points_at_distance {
            let other_range = num_nanobots_in_range_from(&other_point, &nanobots);

            if other_range >= range {
                println!("Vertex: {:?}   num in range: {}     manhattan_distance: {}", other_point, other_range, other_point.manhattan_distance_from(Coord::new(0, 0, 0)));
                cont = true;
            }

            if other_range > range {
                range = other_range;
                point = other_point;
                distance = 0;
            }
        }

    }
}

fn num_nanobots_in_range_from(coord: &Coord, nanobots: &Vec<Nanobot>) -> isize {
    let mut num_nanobots_in_range = 0;
    for nanobot in nanobots.iter() {
        if coord.manhattan_distance_from(nanobot.pos) <= nanobot.range {
            num_nanobots_in_range += 1;
        }
    }

    return num_nanobots_in_range;
}

fn read_inputs<'a>(filename: &str) -> Vec<Nanobot> {
    let mut ret = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
    let lines: Vec<&str> = file_contents.split("\n").collect();

    for line in lines {
        let regex = "pos=<(-?\\d+),(-?\\d+),(-?\\d+)>, r=(\\d+)";
        let cap = Regex::new(regex).unwrap().captures_iter(line).next().expect("Error in regex");

        let pos = Coord::new(cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap());
        let nanobot = Nanobot{pos, range: cap[4].parse().unwrap()};
        ret.push(nanobot);
    }

    return ret;
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Nanobot {
    pos: Coord,
    range: isize
}

impl Nanobot {
    fn vertexes_from(&self) -> HashSet<Coord> {
        let mut ret = HashSet::new();

        ret.insert(Coord::new(self.pos.x + self.range, self.pos.y, self.pos.z));
        ret.insert(Coord::new(self.pos.x - self.range, self.pos.y, self.pos.z));
        ret.insert(Coord::new(self.pos.x, self.pos.y + self.range, self.pos.z));
        ret.insert(Coord::new(self.pos.x, self.pos.y - self.range, self.pos.z));
        ret.insert(Coord::new(self.pos.x, self.pos.y, self.pos.z + self.range));
        ret.insert(Coord::new(self.pos.x, self.pos.y, self.pos.z - self.range));

        return ret;
    }
}

impl fmt::Debug for Nanobot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return writeln!(f, "pos={:?}   range={}", self.pos, self.range);
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

    fn points_at_distance(&self, dist: isize) -> HashSet<Coord> {
        let mut ret = HashSet::new();

        for dx in 0 .. dist+1 {
            let y_dist = dist - dx.abs();

            for dy in 0 .. y_dist + 1 {
                let z_dist = y_dist - dy.abs();

//                for dz in 0 .. z_dist + 1 {
                for dz in z_dist .. z_dist + 1 {
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
        }
        return ret;
    }



}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{},{})", self.x, self.y, self.z);
    }
}


