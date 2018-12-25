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

    let inputs = read_inputs(filename);
    let stars = inputs.0;
    let min_id = inputs.1;
    let max_id = inputs.2;

    let mut pairs = HashSet::new();
    // Check which ones are pairs
    for i in min_id .. max_id + 1 {
        for j in i + 1 .. max_id + 1 {
            let star1: &Star = stars.get(i as usize).unwrap();
            let star2: &Star = stars.get(j as usize).unwrap();

            if star1.pos.manhattan_distance_from(star2.pos) <= 3 {
                pairs.insert(Pair{id1: i, id2: j});
            }
        }

    }

    println!("Pairs size: {}", pairs.len());

    if small_input {
        println!("{:?}", pairs);
    }

    let mut singles = HashSet::new();
    for i in min_id .. max_id + 1 {
        singles.insert(i);
    }
    for pair in pairs.clone() {
        singles.remove(&pair.id1);
        singles.remove(&pair.id2);
    }


    let mut constellations = HashSet::new();

    while pairs.len() > 0 {  // Constellation loop
        constellations.insert(get_one_constellation(&mut pairs));
    }

    println!("Constellations");
    println!("{:?}", constellations);
    println!("{} constellations", constellations.len());

    println!("Singles");
    println!("{:?}", singles);
    println!("{} singles", singles.len());


    println!("\nTotal: {}", constellations.len() + singles.len());


}

fn get_one_constellation(pairs: &mut HashSet<Pair>) -> Constellation {
    let mut constellation = Constellation::new();

    let mut cont = true;
    while cont {
        cont = false;
        for current_pair in pairs.clone().iter() {
            if constellation.len() == 0 || constellation.contains_id(&current_pair.id1) || constellation.contains_id(&current_pair.id2) {
                constellation.add_id(current_pair.id1);
                constellation.add_id(current_pair.id2);
                pairs.remove(current_pair);
                cont = true;
            }
        }
    }

    return constellation;
}




fn read_inputs<'a>(filename: &str) -> (Vec<Star>, isize, isize) {   // Vector, min_id, max_id
    let mut ret = Vec::new();

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut min_id = std::isize::MAX;
    let mut max_id = std::isize::MIN;

    for line in lines {
        let regex = "(-?\\d+),(-?\\d+),(-?\\d+),(-?\\d+)";
        let cap = Regex::new(regex).unwrap().captures_iter(line).next().expect("Error in regex");

        let pos = Coord::new(cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap(), cap[4].parse().unwrap());

        let star = Star::new(pos);


        if star.id < min_id {
            min_id = star.id;
        }

        if star.id > max_id {
            max_id = star.id;
        }

        ret.push(star);
    }

    return (ret, min_id, max_id);
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Constellation {
    ids: Vec<isize>
}

impl Constellation {
    fn new() -> Constellation {
        return Constellation{ids: Vec::new()}
    }

    fn add_id(&mut self, id: isize) {
        if !self.contains_id(&id) {
            self.ids.push(id);
        }
    }

    fn len(&self) -> usize {
        return self.ids.len();
    }

    fn contains_id(&self, id: &isize) -> bool {
        return self.ids.contains(id);
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Pair {
    id1: isize,
    id2: isize,
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{},{}]", self.id1, self.id2);
    }
}


#[derive(Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Star {
    id: isize,
    pos: Coord,
}

static mut NEXT_STAR_ID: isize = 0;
impl Star {
    fn new(pos: Coord) -> Star {
        unsafe {
            let ret = Star{id: NEXT_STAR_ID, pos};

            NEXT_STAR_ID += 1;
            return ret;
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
    t: isize,
}

impl Coord {
    fn new(x: isize, y: isize, z: isize, t: isize) -> Coord {
        return Coord{x, y, z, t}
    }

    fn manhattan_distance_from(&self, other: Coord) -> isize {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        let z_dist = (self.z - other.z).abs();
        let t_dist = (self.t - other.t).abs();

        return x_dist + y_dist + z_dist + t_dist;
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{},{},{})", self.x, self.y, self.z, self.t);
    }
}


