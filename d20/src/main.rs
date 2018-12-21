
use std::fs;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let small_input = false;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut contents = HashMap::new();
    let world = read_inputs(filename, &mut contents);
    println!("{:?}", world);

    let shortest_paths = world.find_shortest_paths();

    let mut furthest_distance = 0;
    let mut furthest_point = world.starting_point;
    let mut num_thousand_distance = 0;
    for (p, dist) in shortest_paths {
        if dist > furthest_distance {
            furthest_distance = dist;
            furthest_point = p;
        }
        if dist >= 1000 {
            num_thousand_distance += 1;
        }
    }


    println!("Furthest point is {:?} with a distance of {}", furthest_point, furthest_distance);
    println!("There are {} rooms with at least 1000 distance", num_thousand_distance);
}

fn read_inputs<'a>(filename: &str, contents: &'a mut HashMap<Coord, Contents>) -> World<'a> {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
    let mut world = World::new(contents);

    add_contents(&mut world, &file_contents[1 .. file_contents.len() - 1], &Coord::new(0, 0));

    return world;
}

fn add_contents<'a>(world: &'a mut World, input: &str, starting_point: &Coord) -> Coord {
    //println!("Input: {}", input);

    let paren_idx_opt = input.find("(");
    let mut split_idx_opt = input.find("|");

    if paren_idx_opt.is_none() && split_idx_opt.is_none() {
        return add_contents_in_line(world, input, starting_point);
    } else if paren_idx_opt.is_some() && split_idx_opt.is_none() {
        return add_contents_from_paren(world, input, starting_point, paren_idx_opt.unwrap());
    } else if paren_idx_opt.is_none() && split_idx_opt.is_some() {
        return add_contents_from_split(world, input, starting_point, split_idx_opt.unwrap());
    } else {
        let mut idx = 0;
        let mut paren_level = 0;

        split_idx_opt = None;
        while idx < input.len() {
            let c = &input[idx..idx+1];

            if c == ")" {
                paren_level -= 1;
            }
            if c == "(" {
                paren_level += 1;
            }
            if c == "|" && paren_level == 0 {
                split_idx_opt = Some(idx);
            }
            idx += 1;
        }

        if split_idx_opt.is_some() {
            return add_contents_from_split(world, input, starting_point, split_idx_opt.unwrap());
        } else {
            return add_contents_from_paren(world, input, starting_point, paren_idx_opt.unwrap());
        }
    }
}

fn add_contents_from_paren<'a>(world: &'a mut World, input: &str, starting_point: &Coord, paren_idx: usize) -> Coord {   // returns end point
    let before_str = &input[0..paren_idx];

    let mut idx = paren_idx;
    let mut paren_level = 1;

    while paren_level != 0 {
        idx += 1;
        let c = input.get(idx .. idx + 1).unwrap();

        if c == ")" {
            paren_level -= 1;
        }
        if c == "(" {
            paren_level += 1;
        }
    }

    let mid_str = &input[paren_idx + 1 .. idx];

    let after_str = &input[idx + 1 .. input.len()];

    let midpoint1 = add_contents(world, before_str, starting_point);
    let midpoint2 = add_contents(world, mid_str, &midpoint1);
    let endpoint = add_contents(world, after_str, &midpoint2);

    return endpoint;
}

fn add_contents_from_split<'a>(world: &'a mut World, input: &str, starting_point: &Coord, split_idx: usize) -> Coord {   // returns end point
    let before_str = &input[0..split_idx];
    let after_str = &input[split_idx + 1 .. input.len()];

    add_contents(world, before_str, starting_point);
    add_contents(world, after_str, starting_point);

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

    fn find_shortest_paths(&self) -> HashMap<Coord, usize> {
        let mut vec = Vec::new();
        let mut distances = HashMap::new();

        vec.push(self.starting_point);
        distances.insert(self.starting_point, 0);

        while vec.len() != 0 {
            let current = vec.pop().unwrap();
            let current_distance = *distances.get(&current).unwrap();

            let navigable_points = self.get_navigable_points_from(&current);

            for nav_point in navigable_points {
                let nav_distance = current_distance + 1;
                if !distances.contains_key(&nav_point) || *distances.get(&nav_point).unwrap() > nav_distance {
                    distances.insert(nav_point, nav_distance);
                    vec.push(nav_point);
                }
            }
        }

        return distances;
    }

    fn get_navigable_points_from(&self, start: &Coord) -> Vec<Coord> {
        let mut ret = Vec::new();

        let directions = vec![Direction::North, Direction::West, Direction::East, Direction::South];
        for dir in directions.iter() {
            let door_coord = start.mv(*dir);
            let door_opt = self.contents.get(&door_coord);
            if door_opt.is_some() && *door_opt.unwrap() == Contents::Door {
                let room_coord = door_coord.mv(*dir);
                ret.push(room_coord);
            }
        }
        return ret;
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
        let letter;

        match self {
            Contents::Door => letter = " ",
            Contents::Room => letter = " ",
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
        let letter ;

        match self {
            Direction::North => letter = "N",
            Direction::South => letter = "S",
            Direction::East => letter = "E",
            Direction::West => letter = "W"
        }

        return write!(f, "{}", letter);
    }
}










