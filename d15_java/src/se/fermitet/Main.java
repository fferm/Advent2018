package se.fermitet;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.*;

public class Main {

    public static void main(String[] args) throws Exception{
        new Main().run();
    }

    private void run() throws Exception {
        boolean smallInput = true;
        String filename;
        if (smallInput) {
            filename = "input_small.txt";
        } else {
            filename = "input.txt";
        }

        Sim sim = readInput(filename);

        System.out.println(sim);

        sim.runRound();
        System.out.println(sim);
    }

    private Sim readInput(String filename) throws Exception {
        Sim sim = new Sim();

        BufferedReader br = new BufferedReader(new FileReader(new File(filename)));

        String st;
        int line = 0;
        while ((st = br.readLine()) != null) {
            for (int x = 0; x < st.length(); x++) {
                Coord c = new Coord(x, line);

                char ch = st.charAt(x);
                if (ch == '#') {
                    sim.walls.add(c);
                } else if (ch == 'E') {
                    sim.players.add(new Player(c, PlayerType.ELF));
                } else if (ch == 'G') {
                    sim.players.add(new Player(c, PlayerType.GOBLIN));
                }

                if (x > sim.size.x - 1) {
                    sim.size = new Coord(x + 1, sim.size.y);
                }
            }
            if (line > sim.size.y - 1) {
                sim.size = new Coord(sim.size.x, line + 1);
            }

            line++;
        }

        return sim;
    }
}

class Sim {
    List<Player> players = new ArrayList<Player>();
    Set<Coord> walls = new HashSet<Coord>();
    Coord size = new Coord(0, 0);

    void runRound() {
        // TODO: Sort

        for (Player player : players) {
            this.runRoundForPlayer(player);
        }
    }

    void runRoundForPlayer(Player player) {
        // Move
        Coord movePos = this.positionToMoveTo(player);
        if (movePos != null) {
            player.moveTo(movePos);
        }

        // Attack
    }

    Coord positionToMoveTo(Player player) {
//        if self.player_in_range_of_enemy(player) {
//            return None;
//        }

        if (this.playerInRangeOfEnemy(player)) {
            return null;
        }

//
//        let mut routes = HashMap::new();
//        let mut vec = Vec::new();
//        routes.insert(player.pos.get(), Route::create_initial(player.pos.get(), &mut vec));
//
//        let mut positions = vec![player.pos.get()];
//
//        let mut shortest_path_length = std::usize::MAX;
//
//        while positions.len() > 0 {
//            let current_pos = positions.pop().unwrap();
//            let current_route = routes.get(&current_pos).unwrap();
//
//            let coords_in_range = current_pos.coords_in_range();
//
//            for potential_move in coords_in_range.iter() {
//                if self.walls.contains(&potential_move) {
//                    continue;
//                }
//
//                if self.get_player_id_at(&potential_move).is_some() {
//                    continue;
//                }
//
//                let mut vec1 = Vec::new();
//                let mut route_to = current_route.create_from_and_add(*potential_move, &mut vec1);
//
//                if route_to.len() > shortest_path_length {
//                    continue;
//                }
//
//                if routes.contains_key(&potential_move) && routes.get(&potential_move).unwrap().len() < route_to.len() {
//                    // TODO: Välj rätt väg om det finns olika vägar till samma ställe
//                    // Tror det är löst i och med reading order på coords_in_range
//                    continue;
//                }
//
//                if self.position_in_range_of_enemy(*potential_move,  &player.player_type) {
//                    shortest_path_length = route_to.len();
//                    // Välj rätt !!!
//                    return Some(route_to.get_first_step());
//                }
//
//                routes.insert(potential_move.clone(), route_to);
//                positions.push(potential_move.clone());
//            }
//        }
//
//        println!("shortest_path_length: {}    {:?}", shortest_path_length, routes);
//
//        return None;

        return null;

    }

    boolean playerInRangeOfEnemy(Player player) {
        return this.positionInRangeOfEnemy(player.pos, player.type);
    }

    boolean positionInRangeOfEnemy(Coord pos, PlayerType friendlyPlayerType) {
        return this.players.stream()
                .filter(p -> p.type != friendlyPlayerType)
                .filter(enemy -> pos.manhattanDistanceFrom(enemy.pos) <= 1)
                .count() >= 1;
    }


    Player getPlayerAt(Coord c) {
        for (Player player : players) {
            if (player.pos.equals(c)) return player;
        }
        return null;
    }

    @Override
    public String toString() {
        StringBuffer buf = new StringBuffer();

        for (int y = 0; y < this.size.y; y++) {
            for (int x = 0; x < this.size.x; x++) {
                Coord c = new Coord(x, y);

                String s = " ";

                if (walls.contains(c)) {
                    s = "#";
                }

                Player potentialPlayer = getPlayerAt(c);
                if (potentialPlayer != null) {
                    s = potentialPlayer.type == PlayerType.ELF ? "E" : "G";
                }

                buf.append(s);
            }
            buf.append("\n");
        }
        buf.append("\n");

        return buf.toString();
    }
}

class Player {
    Coord pos;
    PlayerType type;
    int hitPoints;
    int attackPower;

    Player(Coord pos, PlayerType type) {
        super();
        this.pos = pos;
        this.type = type;
        this.hitPoints = 200;
        this.attackPower = 3;
    }

    void moveTo(Coord pos) {
        this.pos = pos;
    }
}

enum PlayerType {
    ELF, GOBLIN
}

class Coord {
    int x;
    int y;

    Coord(int x, int y) {
        super();
        this.x = x;
        this.y = y;
    }

    int manhattanDistanceFrom(Coord other) {
        int x_dist = Math.abs(this.x - other.x);
        int y_dist = Math.abs(this.y - other.y);

        return x_dist + y_dist;
    }

    @Override
    public String toString() {
        return "(" + x + "," + y + ")";
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Coord coord = (Coord) o;
        return x == coord.x &&
                y == coord.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}
