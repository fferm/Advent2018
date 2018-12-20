
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
//use regex::Regex;
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

    let mut contents = HashMap::new();
    let mut world = read_inputs(filename, &mut contents);
    println!("{:?}", world);
}

fn read_inputs<'a>(filename: &str, contents: &'a mut HashMap<Coord, Contents>) -> World<'a> {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
    let mut world = World::new(contents);

    add_contents(&mut world, &file_contents[1 .. file_contents.len() - 1], &Coord::new(0, 0));

    return world;
}

fn add_contents<'a>(world: &'a mut World, input: &str, starting_point: &Coord) -> Coord {
    println!("Input: {}   start: {:?}", input, starting_point);

    let paren_idx_opt = input.find("(");
    let split_idx_opt = input.find("|");

    if paren_idx_opt.is_none() && split_idx_opt.is_none() {
        return add_contents_in_line(world, input, starting_point);
    } else if paren_idx_opt.is_some() && split_idx_opt.is_none() {
        return add_contents_from_paren(world, input, starting_point, paren_idx_opt.unwrap());
    } else if paren_idx_opt.is_none() && split_idx_opt.is_some() {
        return add_contents_from_split(world, input, starting_point, split_idx_opt.unwrap());
    } else {
        let paren_idx = paren_idx_opt.unwrap();
        let split_idx = split_idx_opt.unwrap();

        if paren_idx < split_idx {
            return add_contents_from_paren(world, input, starting_point, paren_idx_opt.unwrap());
        } else {
            return add_contents_from_split(world, input, starting_point, split_idx_opt.unwrap());
        }
    }

}

fn add_contents_from_paren<'a>(world: &'a mut World, input: &str, starting_point: &Coord, paren_idx: usize) -> Coord {   // returns end point
    let before_str = &input[0..paren_idx];
    let after_str = &input[paren_idx + 1 .. input.len() - 1];

    let midpoint = add_contents(world, before_str, starting_point);
    let endpoint = add_contents(world, after_str, &midpoint);

    return endpoint;
}

fn add_contents_from_split<'a>(world: &'a mut World, input: &str, starting_point: &Coord, split_idx: usize) -> Coord {   // returns end point
    let before_str = &input[0..split_idx];
    let after_str = &input[split_idx + 1 .. input.len()];

    let midpoint = add_contents(world, before_str, starting_point);
    let endpoint = add_contents(world, after_str, starting_point);

    return starting_point.clone();
}

fn add_contents_in_line<'a>(world: &'a mut World, input: &str, starting_point: &Coord) -> Coord {   // returns end point
    let mut current_point = starting_point.clone();

    let mut idx = 0;
    while idx < input.len() {
        let c =input.get(idx..idx+1).unwrap();

        if "NESW".contains(c) {
            let direction = Direction::from_input(c);

            let door_coord = current_point.mv(direction);
            world.insert(&door_coord, Contents::Door);

            let room_coord = door_coord.mv(direction);
            world.insert(&room_coord, Contents::Room);

            current_point = room_coord.clone();
            idx += 1;
        }
    }
    return current_point;
}

struct World<'a> {
    contents: &'a mut HashMap<Coord, Contents>,
    starting_point: Coord,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize
}

impl<'a> World<'a> {
    fn new(contents: &'a mut HashMap<Coord, Contents>) -> World<'a> {
        let starting_point = Coord::new(0, 0);
        let mut world = World{contents, starting_point, x_min: 0, x_max: 0, y_min: 0, y_max: 0};

        world.insert(&starting_point, Contents::Start);

        return world;
    }

    fn get_contents(&self, target: &Coord) -> Contents {
        return *self.contents.get(target).unwrap_or(&Contents::Wall);
    }

    fn insert(&mut self, coord: &Coord, state: Contents) {
        self.contents.insert(coord.clone(), state.clone());

        if coord.x < self.x_min {
            self.x_min = coord.x;
        }
        if coord.x > self.x_max {
            self.x_max = coord.x;
        }
        if coord.y < self.y_min {
            self.y_min = coord.y;
        }
        if coord.y > self.y_max {
            self.y_max = coord.y;
        }
    }
}

impl<'a> fmt::Debug for World<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in self.y_min - 1..self.y_max + 2 {
            for x in self.x_min - 1..self.x_max + 2 {
                let coord = Coord::new(x, y);

                let contents = self.get_contents(&coord);

                ret.push_str(&format!("{:?}", contents)[..]);
            }
            ret.push_str(&"\n");
        }

        ret.push_str(&"\n");

        return write!(f, "{}", ret);
    }
}



#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Contents {
    Door,
    Room,
    Wall,
    Start
}

impl fmt::Debug for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut letter = " ";

        match self {
            Contents::Door => letter = "+",
            Contents::Room => letter = ".",
            Contents::Wall => letter = "#",
            Contents::Start => letter = "X"
        }

        return write!(f, "{}", letter);
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    y: isize,
    x: isize,
}


impl Coord {
    fn new(x: isize, y:isize) -> Coord{
        return Coord{x, y};
    }

    fn mv(&self, dir: Direction) -> Coord {
        match dir {
            Direction::West => Coord { x: self.x - 1, y: self.y },
            Direction::East => Coord { x: self.x + 1, y: self.y },
            Direction::North => Coord { x: self.x, y: self.y - 1 },
            Direction::South => Coord { x: self.x, y: self.y + 1 }
        }
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{})", self.x, self.y);
    }
}


#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn from_input(input: &str) -> Direction {
        match input {
            "N" => return Direction::North,
            "E" => return Direction::East,
            "W" => return Direction::West,
            "S" => return Direction::South,
            _ => panic!("Unknown direction input: {}", input)
        }
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut letter = " ";

        match self {
            Direction::North => letter = "N",
            Direction::South => letter = "S",
            Direction::East => letter = "E",
            Direction::West => letter = "W"
        }

        return write!(f, "{}", letter);
    }
}










