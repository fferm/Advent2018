use std::fs;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let small_input = false;
    let show_anyway = true;

    let filename: &str;
    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut sim = read_inputs(filename);
    println!("{:?}", sim);

    let mut i = 0;
    while i < 10000 {
        sim.tic();
        if small_input || show_anyway {
            println!("{:?}", sim);
        }

        i += 1;

        println!("Generation: {}    num_carts: {}", i, sim.carts.len());

    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord {
    y: isize,
    x: isize
}

impl Coord {
    fn mv(&self, dir: Direction) -> Coord {
        match dir {
            Direction::Left => Coord{x: self.x - 1, y: self.y},
            Direction::Right => Coord{x: self.x + 1, y: self.y},
            Direction::Up => Coord{x: self.x, y: self.y - 1},
            Direction::Down => Coord{x: self.x, y: self.y + 1}
        }
    }

    fn manhattan_distance(&self, other: Coord) -> isize {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();

        return x_dist + y_dist;
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum TurnDirection {
    Left,
    Straight,
    Right
}

impl TurnDirection {
    fn new_direction(&self, old_cart_direction:Direction) -> Direction {
        match *self {
            TurnDirection::Straight => old_cart_direction,
            TurnDirection::Left => {
                match old_cart_direction {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up
                }
            },
            TurnDirection::Right => {
                match old_cart_direction {
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Down
                }
            }
        }
    }
    fn next(&self) -> TurnDirection {
        match *self {
            TurnDirection::Left => TurnDirection::Straight,
            TurnDirection::Straight => TurnDirection::Right,
            TurnDirection::Right => TurnDirection::Left,
        }
    }
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Cart {
    pos: Cell<Coord>,
    direction: Cell<Direction>,
    next_turn: Cell<TurnDirection>,
    symbol: Cell<char>
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Track {
    pos: Coord,
    directions: Vec<Direction>,
    symbol: char
}

struct Sim {
    tracks: HashMap<Coord, Track>,
    carts: Vec<Cart>
}

impl Sim {
    fn minmax(&self) -> (isize, isize, isize, isize) { // min_x, max_x, min_y, max_y
        let mut min_x = std::isize::MAX;
        let mut min_y = std::isize::MAX;
        let mut max_x = std::isize::MIN;
        let mut max_y = std::isize::MIN;

        for (coord, _track) in &self.tracks {
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.x < min_x {
                min_x = coord.x;
            }
            if coord.y > max_y {
                max_y = coord.y;
            }
            if coord.y < min_y {
                min_y = coord.y;
            }
        }

        for cart in &self.carts {
            if cart.pos.get().x > max_x {
                max_x = cart.pos.get().x;
            }
            if cart.pos.get().x < min_x {
                min_x = cart.pos.get().x;
            }
            if cart.pos.get().y > max_y {
                max_y = cart.pos.get().y;
            }
            if cart.pos.get().y < min_y {
                min_y = cart.pos.get().y;
            }
        }
        return (min_x, max_x, min_y, max_y)
    }
    fn tic(&mut self) {
        &self.carts.sort();

        for cart in &self.carts {
            let current_track = self.tracks.get(&cart.pos.get()).unwrap();

            // Turn if needed
            if current_track.symbol == '/' || current_track.symbol == '\\' {
                let dir1 = current_track.directions.get(0).unwrap();
                let dir2 = current_track.directions.get(1).unwrap();
                if cart.direction.get() == (*dir1).opposite() {
                    cart.direction.set(*dir2);
                } else if cart.direction.get() == (*dir2).opposite() {
                    cart.direction.set(*dir1);
                } else {
                    panic!("Unknown cart direction: {:?}", cart);
                }
            } else if current_track.symbol == '+' {
                cart.direction.set(cart.next_turn.get().new_direction(cart.direction.get()));
                cart.next_turn.set(cart.next_turn.get().next());
            }
            match cart.direction.get() {
                Direction::Up => cart.symbol.set('^'),
                Direction::Left => cart.symbol.set('<'),
                Direction::Right => cart.symbol.set('>'),
                Direction::Down => cart.symbol.set('v')
            }

            // Move in new direction
            let new_coord = cart.pos.get().mv(cart.direction.get());

            cart.pos.set(new_coord);

            for other_cart in &self.carts {
                if other_cart == cart {
                    continue;
                }
                let manhattan = cart.pos.get().manhattan_distance(other_cart.pos.get());
                if manhattan < 2 {
                    println!("Manhattan distance {} between {:?} and {:?}", manhattan, cart.pos.get(), other_cart.pos.get());
                }
                if other_cart.pos.get() == cart.pos.get() {
                    println!("Found collision at: {:?}", cart.pos.get());
                    panic!();
                }
            }

        }
    }

    fn get_cart_at(&self, coord: Coord) -> Option<Cart> {
        for cart in &self.carts {
            if cart.pos.get() == coord {
                return Some(cart.clone());
            }
        }
        return None;
    }
}
impl fmt::Debug for Sim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let minmax = self.minmax();

        let min_x = minmax.0;
        let max_x = minmax.1;
        let min_y = minmax.2;
        let max_y = minmax.3;

        let mut ret = "".to_owned();
//        print!("{}[2J", 27 as char);

        for y in min_y .. max_y + 1 {
            for x in min_x .. max_x + 1 {
                let coord = Coord{x: x, y: y};
                let cart_at_coord = self.get_cart_at(coord);
                if cart_at_coord.is_some() {
                    ret.push(cart_at_coord.unwrap().symbol.get())
                } else if self.tracks.contains_key(&coord) {
                    ret.push(self.tracks.get(&coord).unwrap().symbol)
                } else {
                    ret.push(' ');
                }
            }
            ret.push_str(&"\n");
        }

        return write!(f, "{}", ret);
    }
}



fn read_inputs(filename: &str) -> Sim {
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut chars = HashMap::new();

    let mut y = 0;
    for line in lines {
        for x in 0..line.len() {
            let c = line.get(x..x+1).unwrap();
            if c != " " && c != "\r" && c!= "\n" {
                chars.insert(Coord{x: x as isize, y: y}, c);
            }
        }

        y += 1;
    }

    let mut tracks = HashMap::new();
    let mut carts = Vec::new();

    for (coord, c) in chars.clone() {
        if c == ">" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Left, Direction::Right], symbol: '-'});
            carts.push(Cart{pos: Cell::new(coord), direction: Cell::new(Direction::Right), next_turn: Cell::new(TurnDirection::Left), symbol: Cell::new('>')});
        } else if c == "<" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Left, Direction::Right], symbol: '-'});
            carts.push(Cart{pos: Cell::new(coord), direction: Cell::new(Direction::Left), next_turn: Cell::new(TurnDirection::Left), symbol: Cell::new('<')});
        } else if c == "v" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Up, Direction::Down], symbol: '|'});
            carts.push(Cart{pos: Cell::new(coord), direction: Cell::new(Direction::Down), next_turn: Cell::new(TurnDirection::Left), symbol: Cell::new('v')});
        } else if c == "^" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Up, Direction::Down], symbol: '|'});
            carts.push(Cart{pos: Cell::new(coord), direction: Cell::new(Direction::Up), next_turn: Cell::new(TurnDirection::Left), symbol: Cell::new('^')});
        } else if c == "-" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Left, Direction::Right], symbol: '-'});
        } else if c == "|" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Up, Direction::Down], symbol: '|'});
        } else if c == "+" {
            tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right], symbol: '+'});
        } else if c != "/" && c != "\\" {
            panic!("Unknown char: {}  at {:?}", c, coord);
        }
    }

    for (coord, c) in chars.clone() {
        let to_right = coord.mv(Direction::Right);
        let to_left = coord.mv(Direction::Left);

        if c == "/" {
            if tracks.contains_key(&to_right) && tracks.get(&to_right).unwrap().directions.contains(&Direction::Left) {
                tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Right, Direction::Down], symbol: '/'});
            } else if tracks.contains_key(&to_left) && tracks.get(&to_left).unwrap().directions.contains(&Direction::Right) {
                tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Left, Direction::Up], symbol: '/'});
            } else {
                panic!("Unknown turn {}  at {:?}", c, coord);
            }
        } else if c == "\\" {
            if tracks.contains_key(&to_right) && tracks.get(&to_right).unwrap().directions.contains(&Direction::Left) {
                tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Right, Direction::Up], symbol: '\\'});
            } else if tracks.contains_key(&to_left) && tracks.get(&to_left).unwrap().directions.contains(&Direction::Right) {
                tracks.insert(coord, Track{pos: coord, directions: vec![Direction::Left, Direction::Down], symbol: '\\'});
            } else {
                panic!("Unknown turn {}  at {:?}", c, coord);
            }
        }
    }

    let sim = Sim{tracks:tracks, carts: carts};

    return sim;
}

