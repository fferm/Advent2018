use std::fs;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let mut small_input = true;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut sim = read_inputs(filename);

    println!("{:?}", sim);
}



fn read_inputs(filename: &str) -> Sim {
    let mut sim = Sim{walls: HashSet::new(), players: Vec::new(), x_size: 0, y_size: 0};

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut y = 0;
    for line in lines {
        for x in 0..line.len() {
            let coord = Coord{x: x, y: y};

            let c = line.get(x..x+1).unwrap();

            if c == "#" {
                sim.walls.insert(coord);
            } else if c == "E" {
                sim.players.push(Player::create_elf(coord));
            } else if c == "G" {
                sim.players.push(Player::create_goblin(coord));
            }

            if x > sim.x_size {
                sim.x_size = x;
            }
            if y > sim.y_size {
                sim.y_size = y
            }

        }

        y += 1;
    }

    return sim;
}

//#[derive(Debug)]
struct Sim {
    players: Vec<Player>,
    walls: HashSet<Coord>,
    x_size: usize,
    y_size: usize
}

impl fmt::Debug for Sim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0 .. self.y_size + 1 {
            for x in 0 .. self.x_size + 1 {
                let coord = Coord{x: x, y: y};
                let mut ch = " ";
                if self.walls.contains(&coord) {
                    ch = "#";
                }

                for player in &self.players {
                    if player.pos.get() == coord {
                        match player.player_type {
                            PlayerType::Elf => ch = "E",
                            PlayerType::Goblin => ch = "G"
                        }
                    }
                }




                ret.push_str(&ch);
            }
            ret.push_str(&"\n");
        }

        return write!(f, "{}", ret);
    }
}


#[derive(Debug)]
struct Player {
    player_type: PlayerType,
    pos: Cell<Coord>,
    hit_points: Cell<isize>,
    attack_power: isize
}

impl Player {
    fn create_elf(pos: Coord) -> Player {
        return Player{player_type: PlayerType::Elf, pos: Cell::new(pos), hit_points: Cell::new(200), attack_power: 3}
    }
    fn create_goblin(pos: Coord) -> Player {
        return Player{player_type: PlayerType::Goblin, pos: Cell::new(pos), hit_points: Cell::new(200), attack_power: 3}
    }
}

#[derive(Debug)]
enum PlayerType {
    Elf,
    Goblin
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    y: usize,
    x: usize
}

