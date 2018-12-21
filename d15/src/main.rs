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

    let mut sim = read_inputs(filename);
    println!("{:?}", sim);

    sim.run_round();
    println!("{:?}", sim);
}



fn read_inputs<'a> (filename: &str) -> Sim {
    let mut walls = HashSet::new();
    let mut x_size = 0;
    let mut y_size = 0;

    let mut players_vec = Vec::new();

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
                players_vec.push(Player::create_elf(coord));
            } else if c == "G" {
                players_vec.push(Player::create_goblin(coord));
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

    return Sim{walls, players: Rc::new(RefCell::new(players_vec)), x_size, y_size};
}

struct Sim {
    players: Rc<RefCell<Vec<Player>>>,
    walls: HashSet<Coord>,
    x_size: usize,
    y_size: usize
}

impl<'a> Sim {
    fn run_round(&self) {
        self.players.borrow_mut().sort();

        for player in self.players.borrow_mut().iter_mut() {
            self.run_round_for_player(player)
        }
    }

    fn run_round_for_player(&self, player: &mut Player) {
        // Move
        let move_pos = self.position_to_move_to(player);
        if move_pos.is_some() {
            player.pos = move_pos.unwrap();
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

        //        let mut routes = Routes::create();
//        routes.add_route(Route::create_initial(player.pos.get()));
////        routes.insert(player.pos.get(), Route::create_initial(player.pos.get()));
//
//        let mut positions = vec![player.pos.get()];
//
//        let mut shortest_path_length = std::usize::MAX;
//        while positions.len() > 0 {
//            let current_pos = positions.pop().unwrap();
//
//            let current_route = routes.get_route_to(&current_pos).unwrap();
////            let current_route = routes.get(&current_pos).unwrap();
//
//            let coords_in_range = current_pos.coords_in_range();
//            for potential_move in coords_in_range {
//                if self.walls.contains(&potential_move) {
//                    continue;
//                }
//                if self.get_player_at(potential_move).is_some() {
//                    continue;
//                }
//
//                let mut route_to = current_route.create_from_and_add(potential_move);
//
//                if route_to.len() > shortest_path_length {
//                    continue;
//                }
//
//                let prev_route_to = routes.get_route_to(&potential_move);
////                if routes.contains_key(&potential_move) && routes.get(&potential_move).unwrap().len() < route_to.len() {
//                if prev_route_to.is_some() && prev_route_to.unwrap().len() < route_to.len() {
//                    // TODO: Välj rätt väg om det finns olika vägar till samma ställe
//                    // Tror det är löst i och med reading order på coords_in_range
//                    continue;
//                }
//
//                if self.position_in_range_of_enemy(potential_move,  &player.player_type) {
//                    shortest_path_length = route_to.len();
//                    // Välj rätt !!!
////                    return Some(*route_to.get(0).unwrap());
//                }
//
//                routes.add_route(route_to);
//                positions.push(potential_move);
//            }
//        }
//
//        println!("shortest_path_length: {}    {:?}", shortest_path_length, routes);
//
//        return Some(player.pos.get().mv(Direction::Right));

        return None;
    }

    fn player_in_range_of_enemy(&self, player: &Player) -> bool {
        return self.position_in_range_of_enemy(player.pos, &player.player_type);
    }

    fn position_in_range_of_enemy(&self, pos: Coord, friendly_player_type: &PlayerType) -> bool {
        let p = Rc::clone(*self.players);

        for enemy in p.borrow().iter() {
            if enemy.player_type == *friendly_player_type {
                continue;
            }

            if pos.manhattan_distance_from(enemy.pos) == 1 {
                return true;
            }
        }

        return false;
    }

    fn get_player_at(&self, pos: &Coord) -> Option<Player> {
        for player in self.players.borrow().iter() {
            if player.pos == *pos {
                return Some(*player);
            }
        }
        return None;
    }

}

impl<'a> fmt::Debug for Sim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = "".to_owned();

        for y in 0 .. self.y_size + 1 {
            for x in 0 .. self.x_size + 1 {
                let coord = Coord{x: x as isize, y: y as isize};
                let mut ch = " ";

                if self.walls.contains(&coord) {
                    ch = "#";
                }

                let player_at_option = self.get_player_at(&coord);
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


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Player {
    pos: Coord,           // position must be first so that players are sorted in reading order
    player_type: PlayerType,
    hit_points: isize,
    attack_power: isize
}

impl Player {
    fn create_elf(pos: Coord) -> Player {
        return Player{player_type: PlayerType::Elf, pos, hit_points: 200, attack_power: 3};
    }
    fn create_goblin(pos: Coord) -> Player {
        return Player{player_type: PlayerType::Goblin, pos, hit_points: 200, attack_power: 3};
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
//    fn coords_in_range(&self) -> Vec<Coord> {
//        return vec![
//            Coord{x: self.x, y: self.y - 1},    // ordering is important (reading order)
//            Coord{x: self.x - 1, y: self.y},
//            Coord{x: self.x + 1, y: self.y},
//            Coord{x: self.x, y: self.y + 1}
//        ];
//    }

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

//#[derive(Debug)]
//struct Route {
//    steps: Vec<Coord>,
//    enemy_position: Option<Coord>,
//    own_end_position: Option<Coord>
//}

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

//impl Route {
//    fn create_initial(starting_pos: Coord) -> Route {
//        return Route{steps: vec![starting_pos], enemy_position: None, own_end_position: None}
//    }
//
//    fn create_from_and_add(&self, pos: Coord) -> Route {
//        let mut steps = self.steps.clone();
//        steps.push(pos);
//
//        return Route{steps: steps, enemy_position: self.enemy_position.clone(), own_end_position: self.own_end_position.clone() };
//    }
//
//    fn len(&self) -> usize {
//        return self.steps.len() - 1;
//    }
//
//    fn get_end_pos(&self) -> Coord {
//        return *self.steps.get(self.steps.len() - 1).unwrap();
//    }
//}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}


