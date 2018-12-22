
use std::collections::HashMap;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    let small_input = true;

    let depth: isize;
    let target: Coord;

    if small_input {
        depth = 510;
        target = Coord::new(10, 10);
    } else {
        depth = 3558;
        target = Coord::new(15,740);
    }

    let mut world = World::new(target, depth);

    if small_input {
        println!("{:?}", world);
    }
    println!("Risk level is {}", world.get_risk_level());


    let fastest_time = find_fastest_time(world);
    println!("Target can be reached in {} minutes", fastest_time);
}

fn find_fastest_time(world: World) -> isize {
    let start_status = (Coord::new(0, 0), Equip::Torch);
    let target_status = (world.target, Equip::Torch);

    let best_times_for_status: HashMap<(Coord, Equip), isize> = HashMap::new();
    let prio_queue = 
}





struct World {
    geologic_indices: RefCell<HashMap<Coord, isize>>,
    target: Coord,
    depth: isize
}

impl World {
    fn new(target: Coord, depth: isize) -> World {
        let geologic_indices = RefCell::new(HashMap::new());

        return World{target, depth, geologic_indices};
    }

    fn get_geologic_index(&self, coord: &Coord) -> isize {
        if self.geologic_indices.borrow().contains_key(coord) {
            return *self.geologic_indices.borrow().get(coord).unwrap();
        }

        let value = self.calculate_geologic_index(coord);

        self.geologic_indices.borrow_mut().insert(coord.clone(), value);
        return value;
    }

    fn calculate_geologic_index(&self, coord: &Coord) -> isize {
        let mut result = 0;
        if *coord == Coord::new(0, 0) {
            result = 0;
        } else if *coord == self.target {
            result = 0;
        } else if coord.y == 0 {
            result = coord.x * 16807;
        } else if coord.x == 0 {
            result = coord.y * 48271;
        } else {
            let new_coord1 = Coord::new(coord.x - 1, coord.y);
            let new_coord2 = Coord::new(coord.x, coord.y - 1);

            result = self.get_erosion_level(&new_coord1) * self.get_erosion_level(&new_coord2);
        }

        return result;
    }

    fn get_erosion_level(&self, coord: &Coord) -> isize {
        return (self.get_geologic_index(coord) + self.depth) % 20183;
    }

    fn get_type(&self, coord: &Coord) -> Type {
        let erosion_level = self.get_erosion_level(coord);
        let modulo = erosion_level % 3;

        match modulo {
            0 => return Type::Rocky,
            1 => return Type::Wet,
            2 => return Type::Narrow,
            _ => panic!("No type for modulo {} at coordinate {:?}", modulo, coord)
        }
    }

    fn get_risk_level(&mut self) -> isize {
        let mut risk_level = 0;
        for x in 0 .. self.target.x + 1 {
            for y in 0 .. self.target.y + 1 {
                let coord = Coord::new(x, y);
                let typ = self.get_type(&coord);

                match typ {
                    Type::Rocky => risk_level += 0,
                    Type::Narrow => risk_level += 2,
                    Type::Wet => risk_level += 1
                }
            }
        }

        return risk_level;
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0 .. self.target.y + 1 {
            for x in 0 .. self.target.x + 1 {
                let coord = Coord::new(x, y);

                if x == 0 && y == 0 {
                    ret.push_str("M");
                } else if coord == self.target {
                    ret.push_str("T");
                } else {
                    let typ = self.get_type(&coord);
                    ret.push_str(&format!("{:?}", typ)[..]);
                }

            }
            ret.push_str(&"\n");
        }

        ret.push_str(&"\n");

        return write!(f, "{}", ret);
    }
}


#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Type {
    Rocky,
    Narrow,
    Wet
}

impl Type {
    fn can_use_equip(&self, equip: &Equip) -> bool {
        match self {
            Type::Rocky => return *equip == Equip::Climbing || *equip == Equip::Torch,
            Type::Narrow => return *equip == Equip::Neither || *equip == Equip::Torch,
            Type::Wet => return *equip == Equip::Climbing || *equip == Equip::Neither,
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ch = ' ';

        match self {
            Type::Rocky => ch = '.',
            Type::Narrow => ch = '|',
            Type::Wet => ch = '='
        }

        return write!(f, "{}", ch);
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}


#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
enum Equip {
    Torch,
    Climbing,
    Neither
}






