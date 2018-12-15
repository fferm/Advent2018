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

    sim.run_round();
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

impl Sim {
    fn run_round(&mut self) {
        // Sort players
        self.players.sort();

        for player in &self.players {
            println!("Player at: {}, {}", player.pos.get().x, player.pos.get().y);

            // Move
            for enemy in &self.players {
                if enemy.player_type == player.player_type {
                    continue;
                }

                let coords_in_range = enemy.pos.get().coords_in_range();
                for potential_move in coords_in_range {
                    if self.walls.contains(&potential_move) {
                        continue;
                    }

                    let movement_info = self.player_movement_info(player, potential_move, player.pos.get());

                    if !movement_info.0 {
                        println!("Cannot reach {:?}", potential_move);
                        continue;
                    }

                    println!("In range: {:?}   Distance: {}", potential_move, movement_info.1);
                }


            }

            // Attack
        }
    }

    fn player_movement_info(&self, player: &Player, target: Coord, starting_point: Coord) -> (bool, usize) {
        let mut distances: HashMap<Coord, usize> = HashMap::new();
        distances.insert(starting_point, 0);

        let mut positions = vec![starting_point];

        while positions.len() > 0 {
            let current_position = positions.pop().unwrap();
            let current_distance = distances.get(&current_position).unwrap();

            let in_range_from_current = current_position.coords_in_range();

            for candidate in in_range_from_current {
                if self.walls.contains(&candidate) {
                    continue;
                }
                if self.get_player_at(candidate).is_some() {
                    continue;
                }

                if distances.contains_key(&candidate) {
                    continue;
                }

                if candidate == target {
                    return (true, current_distance + 1);
                }

                distances.insert(candidate, current_distance + 1);
                positions.push(candidate);
            }

        }

        return (false, 0);

    }

    fn get_player_at(&self, pos: Coord) -> Option<&Player> {
        for player in &self.players {
            if player.pos.get() == pos {
                return Some(player);
            }
        }
        return None;
    }

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

                let player_at_option = self.get_player_at(coord);
                if player_at_option.is_some() {
                    match player_at_option.unwrap().player_type {
                        PlayerType::Elf => ch = "E",
                        PlayerType::Goblin => ch = "G"
                    }
                }

                ret.push_str(&ch);
            }
            ret.push_str(&"\n");
        }

        return write!(f, "{}", ret);
    }
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Player {
    pos: Cell<Coord>,           // position must be first so that players are sorted in reading order
    player_type: PlayerType,
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum PlayerType {
    Elf,
    Goblin
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Coord {
    y: usize,               // y must be before x so that positions are sorted in reading order
    x: usize
}

impl Coord {
    fn coords_in_range(&self) -> Vec<Coord> {
        return vec![
            Coord{x: self.x + 1, y: self.y},
            Coord{x: self.x - 1, y: self.y},
            Coord{x: self.x, y: self.y + 1},
            Coord{x: self.x, y: self.y - 1}
        ];
    }
}

