package se.fermitet;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

class Sim {
    List<Player> players = new ArrayList<Player>();
    Set<Coord> walls = new HashSet<Coord>();
    Coord size = new Coord(0, 0);


    class Result {
        PlayerType winner;
        int numRounds;
        int totalHitPoints;
    }

    static Sim fromFile(String filename) throws Exception {
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

    Result runFull(boolean debug) {
        int i = 0;
        while (!this.didAnybodyWin().isPresent()) {
            Optional<PlayerType> didAnybodyWin = this.runRound();

            if (!didAnybodyWin.isPresent()) {
                i++;
                if (debug) {
                    System.out.println("After " + i + " rounds");
                    System.out.println(this);
                }
            }
        }

        Result result = new Result();
        result.numRounds = i;
        result.winner = this.didAnybodyWin().get();
        result.totalHitPoints = this.getTotalHitPoints(result.winner);

        return result;

    }

    Optional<PlayerType> runRound() {
        this.sortPlayers();

        for (Player player : players) {
            if (!player.alive) continue;

            if (!didAnybodyWin().isPresent()) {
                runRoundForPlayer(player);
            } else {
                return didAnybodyWin();
            }
        }
        return Optional.empty();
    }

    private void sortPlayers() {
        this.players = this.players.stream()
                .filter(player -> player.alive)
                .sorted()
                .collect(Collectors.toList());
    }

    Optional<PlayerType> runRoundForPlayer(Player player) {
        if (!player.alive) return Optional.empty();
        if (didAnybodyWin().isPresent()) return didAnybodyWin();

        // Move
        Coord movePos = this.positionToMoveTo(player);
        if (movePos != null) {
            player.moveTo(movePos);
        }

        // Attack
        List<Player> allEnemiesInRange = this.allEnemiesInRangeOfPlayer(player, player.type);

        Optional<Player> enemyToAttackOpt = allEnemiesInRange.stream()
                .sorted((Player enemy1, Player enemy2) -> {
                    int hp1 = enemy1.hitPoints;
                    int hp2 = enemy2.hitPoints;

                    if (hp1 == hp2) {
                        Coord pos1 = enemy1.pos;
                        Coord pos2 = enemy2.pos;
                        return pos1.compareTo(pos2);
                    } else {
                        return hp1 - hp2;
                    }
                })
                .findFirst();

        if (enemyToAttackOpt.isPresent()) {
            Player enemy = enemyToAttackOpt.get();
            enemy.hitPoints -= player.attackPower;

            if (enemy.hitPoints <= 0) {
                enemy.alive = false;
            }
        }

        return didAnybodyWin();
    }

    public Stream<Coord> pointsInRangeOfEnemy(Player player) {
        if (!player.alive) return null;
        if (this.playerInRangeOfEnemy(player)) {
            return null;
        }

        return this.players.stream()
                .filter(p -> p.alive)
                .filter(p -> p.type != player.type)
                .flatMap(p -> p.pos.coordsInRange().stream())
                .filter(c -> !this.walls.contains(c));
    }

    Coord positionToMoveTo(Player player) {
        if (!player.alive) return null;
        if (this.playerInRangeOfEnemy(player)) {
            return null;
        }


        Stream<Coord> pointsInRange = this.pointsInRangeOfEnemy(player);
        // TODO
        return null;
        

/*        Stack<Coord> positions = new Stack<>();
        HashMap<Coord, Route> routes = new HashMap();

        positions.add(player.pos);
        routes.put(player.pos, new Route(player.pos));

        int shortestPathLength = Integer.MAX_VALUE;

        while (!positions.isEmpty()) {
            Coord currentPos = positions.pop();
            Route currentRoute = routes.get(currentPos);

            for (Coord potentialMove : currentPos.coordsInRange()) {
                if (this.walls.contains(potentialMove)) {
                    continue;
                }

                if (this.getPlayerAt(potentialMove) != null) {
                    continue;
                }

                Route routeTo = currentRoute.createFromAndAdd(potentialMove);

                if (routeTo.length() > shortestPathLength) {
                    continue;
                }

                if (routes.containsKey(potentialMove) && routes.get(potentialMove).length() <= routeTo.length()) {
                    continue;
                }

                Optional<Player> enemyOpt = this.positionInRangeOfEnemy(potentialMove, player.type);
                if (enemyOpt.isPresent()) {
                    shortestPathLength = routeTo.length();
                    routeTo.enemyPosition = enemyOpt.get().pos;
                    routeTo.ownEndPosition = potentialMove;
                }

                routes.put(potentialMove, routeTo);
                positions.push(potentialMove);
            }
        }

        if (shortestPathLength == Integer.MAX_VALUE) {
            return null;
        }


        final int shortestPath = shortestPathLength;

        List<Route> routeList = routes.entrySet().stream()
                .map(entry -> entry.getValue())
                .filter(route -> route.length() <= shortestPath)
                .filter(route -> route.enemyPosition != null)
                .sorted((Route r1, Route r2)-> r1.enemyPosition.compareTo(r2.enemyPosition))
                .collect(Collectors.toList());

        Coord enemyPosition = routeList.get(0).enemyPosition;

        routeList = routeList.stream()
                .filter(route -> route.enemyPosition.equals(enemyPosition))
                .sorted((Route r1, Route r2) -> r1.ownEndPosition.compareTo(r2.ownEndPosition))
                .collect(Collectors.toList());

        return routeList.get(0).steps.get(1);*/
    }

    boolean playerInRangeOfEnemy(Player player) {
        return this.positionInRangeOfEnemy(player.pos, player.type).isPresent();
    }

    Optional<Player> positionInRangeOfEnemy(Coord pos, PlayerType friendlyPlayerType) {
        return this.players.stream()
                .filter(p -> p.alive)
                .filter(p -> p.type != friendlyPlayerType)
                .filter(enemy -> pos.manhattanDistanceFrom(enemy.pos) <= 1)
                .sorted()
                .findFirst();
    }

    List<Player> allEnemiesInRangeOfPlayer(Player player, PlayerType friendlyPlayerType) {
        return this.players.stream()
                .filter(p -> p.alive)
                .filter(p -> p.type != friendlyPlayerType)
                .filter(enemy -> player.pos.manhattanDistanceFrom(enemy.pos) <= 1)
                .sorted()
                .collect(Collectors.toList());
    }


    Player getPlayerAt(Coord c) {
        for (Player player : players) {
            if (!player.alive) continue;

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

                String s = ".";

                if (walls.contains(c)) {
                    s = "#";
                }

                Player potentialPlayer = getPlayerAt(c);
                if (potentialPlayer != null && potentialPlayer.alive) {
                    s = potentialPlayer.type == PlayerType.ELF ? "E" : "G";
                }

                buf.append(s);
            }
            buf.append("\n");
        }

        for (Player p : this.players) {
            if (!p.alive) continue;

            buf.append(p.type.toString() + " at " + p.pos + " with hit points " + p.hitPoints);
            buf.append("\n");
        }
        buf.append("\n");

        return buf.toString();
    }

    Optional<PlayerType> didAnybodyWin() {
        boolean goblinsLeft = this.players.stream()
                .filter(p -> p.alive)
                .filter(p -> p.type == PlayerType.GOBLIN)
                .findFirst().isPresent();

        boolean elvesLeft = this.players.stream()
                .filter(p -> p.alive)
                .filter(p -> p.type == PlayerType.ELF)
                .findFirst().isPresent();

        if (goblinsLeft && elvesLeft) {
            return Optional.empty();
        } else if (goblinsLeft) {
            return Optional.of(PlayerType.GOBLIN);
        } else {
            return Optional.of(PlayerType.ELF);
        }
    }

    public int getTotalHitPoints(PlayerType winnerType) {
        int ret = 0;
        for (Player p : this.players) {
            if (!p.alive) continue;
            if (p.type != winnerType) continue;

            ret += p.hitPoints;
        }

        return ret;
    }
}
