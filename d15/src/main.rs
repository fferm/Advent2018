use std::fs;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let small_input = true;
    let filename;

    if small_input {
        filename = "input_small.txt";
    } else {
        filename = "input.txt";
    }

    let mut players = HashMap::new();
    let mut sim = read_inputs(filename, &mut players);
    println!("{:?}", sim);

    sim.run_round();
    println!("{:?}", sim);
}



fn read_inputs<'a> (filename: &str, players: &'a mut HashMap<isize, Player>) -> Sim<'a> {
    let mut walls = HashSet::new();
    let mut x_size = 0;
    let mut y_size = 0;

    let file_contents = fs::read_to_string(filename).expect("Error in reading file");

    let lines: Vec<&str> = file_contents.split("\n").collect();

    let mut y = 0;

    for line in lines {
        for x in 0..line.len() {
            let coord = Coord{x: x as isize, y: y as isize};
            let c = line.get(x..x+1).unwrap();

            if c == "#" {
                walls.insert(coord);
            } else if c == "E" {
                let player = Player::create_elf(coord);
                players.insert(player.id, player);
            } else if c == "G" {
                let player = Player::create_goblin(coord);
                players.insert(player.id, player);
            }
            if x > x_size {
                x_size = x;
            }
            if y > y_size {
                y_size = y;
            }
        }

        y += 1;

    }

    return Sim{walls, players, x_size, y_size};
}

struct Sim<'a> {
    players: &'a HashMap<isize, Player>,
    walls: HashSet<Coord>,
    x_size: usize,
    y_size: usize
}

impl<'a> Sim<'a> {
    fn run_round(&self) {

        for id in self.get_player_ids_sorted() {
            self.run_round_for_player(id);
        }
    }


    fn run_round_for_player(&self, id: isize) {
        let player = self.players.get(&id).unwrap();

        // Move
        let move_pos = self.position_to_move_to(player);
        if move_pos.is_some() {
            player.pos.set(move_pos.unwrap());
        }

        // Attack

    }
//    fn run_round(&mut self) {
//        // Sort players
//        self.players.sort();
//
//        for player in &self.players {
//            println!("Player at: {}, {}", player.pos.get().x, player.pos.get().y);
//
//            // Move
//            let move_pos = self.position_to_move_to(player);
//            if move_pos.is_some() {
//                player.pos.set(move_pos.unwrap());
//            }
//
//            /*            for enemy in &self.players {
//                            if enemy.player_type == player.player_type {
//                                continue;
//                            }
//
//                            let coords_in_range = enemy.pos.get().coords_in_range();
//                            for potential_move in coords_in_range {
//                                if self.walls.contains(&potential_move) {
//                                    continue;
//                                }
//
//                                let movement_info = self.player_movement_info(player, potential_move, player.pos.get());
//
//                                if !movement_info.0 {
//                                    println!("Cannot reach {:?}", potential_move);
//                                    continue;
//                                }
//
//                                println!("In range: {:?}   Distance: {}", potential_move, movement_info.1);
//                            }
//
//
//                        }*/
//
//            // Attack
//        }
//
//    }

    fn position_to_move_to(&self, player: &Player) -> Option<Coord> {
        if self.player_in_range_of_enemy(player) {
            return None;
        }

        let mut routes = HashMap::new();
        let mut vec = Vec::new();
        routes.insert(player.pos.get(), Route::create_initial(player.pos.get(), &mut vec));

        let mut positions = vec![player.pos.get()];

        let mut shortest_path_length = std::usize::MAX;

        while positions.len() > 0 {
            let current_pos = positions.pop().unwrap();
            let current_route = routes.get(&current_pos).unwrap();

            let coords_in_range = current_pos.coords_in_range();

            for potential_move in coords_in_range.iter() {
                if self.walls.contains(&potential_move) {
                    continue;
                }

                if self.get_player_id_at(&potential_move).is_some() {
                    continue;
                }

                let mut vec1 = Vec::new();
                let mut route_to = current_route.create_from_and_add(*potential_move, &mut vec1);

                if route_to.len() > shortest_path_length {
                    continue;
                }

                if routes.contains_key(&potential_move) && routes.get(&potential_move).unwrap().len() < route_to.len() {
                    // TODO: Välj rätt väg om det finns olika vägar till samma ställe
                    // Tror det är löst i och med reading order på coords_in_range
                    continue;
                }

                if self.position_in_range_of_enemy(*potential_move,  &player.player_type) {
                    shortest_path_length = route_to.len();
                    // Välj rätt !!!
                    return Some(route_to.get_first_step());
                }

                routes.insert(potential_move.clone(), route_to);
                positions.push(potential_move.clone());
            }
        }

        println!("shortest_path_length: {}    {:?}", shortest_path_length, routes);

        return None;
    }

    fn player_in_range_of_enemy(&self, player: &Player) -> bool {
        return self.position_in_range_of_enemy(player.pos.get(), &player.player_type);
    }

    fn position_in_range_of_enemy(&self, pos: Coord, friendly_player_type: &PlayerType) -> bool {
        for (_, enemy) in self.players {
            if enemy.player_type == *friendly_player_type {
                continue;
            }

            if pos.manhattan_distance_from(enemy.pos.get()) == 1 {
                return true;
            }
        }

        return false;
    }

    fn get_player_id_at(&self, pos: &Coord) -> Option<isize> {
        for (id, player) in self.players {
            if player.pos.get() == *pos {
                return Some(*id);
            }
        }
        return None;
    }

    fn get_player_ids_sorted(&self) -> Vec<isize> {
        let mut positions = Vec::new();

        for player in self.players.values() {
            positions.push(player.pos.get().clone());
        }
        positions.sort();

        let mut ids = Vec::new();
        for position in positions {
            ids.push(self.get_player_id_at(&position).unwrap())
        }

        return ids;
    }


}

impl<'a> fmt::Debug for Sim<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0 .. self.y_size + 1 {
            for x in 0 .. self.x_size + 1 {
                let coord = Coord{x: x as isize, y: y as isize};
                let mut ch = " ";

                if self.walls.contains(&coord) {
                    ch = "#";
                }

                let player_at_option = self.get_player_id_at(&coord);
                if player_at_option.is_some() {
                    let player_type = self.players.get(&player_at_option.unwrap()).unwrap().player_type;

                    match player_type {
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
    id: isize,
    hit_points: Cell<isize>,
    attack_power: isize
}

static mut NEXT_PLAYER_ID: isize = 0;
impl Player {
    fn create_elf(pos: Coord) -> Player {
        return Player{player_type: PlayerType::Elf, pos: Cell::new(pos), hit_points: Cell::new(200), attack_power: 3, id: Player::get_next_player_id()};
    }
    fn create_goblin(pos: Coord) -> Player {
        return Player{player_type: PlayerType::Goblin, pos: Cell::new(pos), hit_points: Cell::new(200), attack_power: 3, id: Player::get_next_player_id()};
    }

    fn get_next_player_id() -> isize {
        let mut ret: isize;

        unsafe {
            NEXT_PLAYER_ID += 1;
            ret = NEXT_PLAYER_ID.clone();
        }

        return ret;
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum PlayerType {
    Elf,
    Goblin
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Coord {
    y: isize,               // y must be before x so that positions are sorted in reading order
    x: isize
}

impl Coord {
    fn coords_in_range(&self) -> Vec<Coord> {
        return vec![
            Coord{x: self.x, y: self.y - 1},    // ordering is important (reading order)
            Coord{x: self.x - 1, y: self.y},
            Coord{x: self.x + 1, y: self.y},
            Coord{x: self.x, y: self.y + 1}
        ];
    }

    fn new(x: isize, y: isize) -> Coord {
        return Coord{x, y}
    }

    fn mv(&self, dir: Direction) -> Coord {
        match dir {
            Direction::Left => Coord{x: self.x - 1, y: self.y},
            Direction::Right => Coord{x: self.x + 1, y: self.y},
            Direction::Up => Coord{x: self.x, y: self.y - 1},
            Direction::Down => Coord{x: self.x, y: self.y + 1}
        }
    }

    fn manhattan_distance_from(&self, other: Coord) -> isize {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();

        return x_dist + y_dist;
    }
}

#[derive(Debug)]
struct Route<'a> {
    steps: &'a Vec<Coord>,
    enemy_position: Option<Coord>,
    own_end_position: Option<Coord>
}

/*impl Copy for Route {

}
impl Clone for Route {
    fn clone(&self) -> Route {
        let steps = self.steps.get().clone();
        let enemy_position = self.enemy_position.clone();
        let own_end_position = self.own_end_position.clone();

        return Route{steps: Cell::new(steps), enemy_position, own_end_position};
    }
}*/

impl<'a> Route<'a> {
    fn create_initial(starting_pos: Coord, vec: &'a mut Vec<Coord>) -> Route<'a> {
        vec.push(starting_pos);
        return Route{steps: &vec, enemy_position: None, own_end_position: None}
    }

    fn create_from_and_add(&mut self, pos: Coord, vec: &'a mut Vec<Coord>) -> Route<'a> {
        for c in self.steps.iter() {
            vec.push(*c);
        }
        vec.push(pos);

        return Route{steps: &vec, enemy_position: self.enemy_position.clone(), own_end_position: self.own_end_position.clone() };
    }

    fn len(&self) -> usize {
        return self.steps.len() - 1;
    }

    fn get_first_step(&self) -> Coord {
        return self.steps.get(1).unwrap().clone();
    }

//    fn get_end_pos(&self) -> Coord {
//        return *self.steps.get(self.steps.len() - 1).unwrap();
//    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}


