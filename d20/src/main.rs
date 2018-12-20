
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
    let mut world = World::new(&mut contents);

//    let mut world = read_inputs(filename, &mut contents);

    world.insert(&Coord::new(5, 3), Contents::Door);
    world.insert(&Coord::new(0, 0), Contents::Room);
    world.insert(&Coord::new(1, 1), Contents::Wall);

    let mut steps = HashSet::new();
    println!("{:?}", world);

    let input = read_inputs(filename, steps);
    println!("{:?}", input)
}

fn read_inputs<'a>(filename: &str, steps: HashSet<Step<'a>>) -> Input<'a> {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    for idx in 0..file_contents.len() {
        let c =file_contents.get(idx..idx+1).unwrap();
        println!("{}", c);
    }
    
    return Input{steps, }




/*    let mut acres = HashMap::new();
    let mut x_size = 0;
    let mut y_size = 0;
    let mut y = 0;
    for line in lines {
        for x in 0..line.len() {
            let coord = Coord{x, y};

            let c = line.get(x..x+1).unwrap();

            if c == "." {
                acres.insert(coord, State::Open);
            } else if c == "#" {
                acres.insert(coord, State::Lumberyard);
            } else if c == "|" {
                acres.insert(coord, State::Trees);
            }

            if x > x_size && acres.contains_key(&coord) {
                x_size = x;
            }

        }
        if y > y_size {
            y_size = y
        }

        y += 1;
    }

    return World{acres, x_size, y_size};*/
}

#[derive(Debug)]
struct Input<'a> {
    steps: HashSet<Step<'a>>,
    head: &'a Step<'a>
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Step<'a> {
    direction: Direction,

    // TODO, flera next behöver vara möjliga
    next: &'a Step<'a>
}

struct World<'a> {
    contents: &'a mut HashMap<Coord, Contents>,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize
}

impl<'a> World<'a> {
    fn new(contents: &'a mut HashMap<Coord, Contents>) -> World<'a> {
        return World{contents, x_min: 0, x_max: 0, y_min: 0, y_max: 0};
    }

    fn get_contents(&self, target: &Coord) -> Contents {
        return *self.contents.get(target).unwrap_or(&Contents::Unknown);
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

        for y in self.y_min..self.y_max + 1 {
            for x in self.x_min..self.x_max + 1 {
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
    Unknown
}

impl fmt::Debug for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut letter = " ";

        match self {
            Contents::Door => letter = "|",
            Contents::Room => letter = ".",
            Contents::Wall => letter = "#",
            Contents::Unknown => letter = " "
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
            Direction::East => Coord { x: self.x - 1, y: self.y },
            Direction::West => Coord { x: self.x + 1, y: self.y },
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










