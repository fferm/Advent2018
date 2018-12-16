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

//    let mut sim = Sim{walls: HashSet::new(), players: Vec::new(), x_size: 0, y_size: 0};
//
//    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
//
//    let lines: Vec<&str> = file_contents.split("\n").collect();
//
//    let mut y = 0;
//    for line in lines {
//        for x in 0..line.len() {
//            let coord = Coord{x: x as isize, y: y as isize};
//
//            let c = line.get(x..x+1).unwrap();
//
//            if c == "#" {
//                sim.walls.insert(coord);
//            } else if c == "E" {
//                sim.players.push(Player::create_elf(coord));
//            } else if c == "G" {
//                sim.players.push(Player::create_goblin(coord));
//            }
//
//            if x > sim.x_size {
//                sim.x_size = x;
//            }
//            if y > sim.y_size {
//                sim.y_size = y
//            }
//
//        }
//
//        y += 1;
//    }
//
//    return sim;

}

class Sim {
    List<Player> players = new ArrayList<Player>();
    Set<Coord> walls = new HashSet<Coord>();
    Coord size = new Coord(0, 0);

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
