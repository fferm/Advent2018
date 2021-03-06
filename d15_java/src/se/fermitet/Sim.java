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
        int elfPower;

        public void print() {
            System.out.println("Combat ends after " + this.numRounds + " full rounds");
            System.out.println(this.winner + " wins with " + this.totalHitPoints + " left");
            System.out.println("Outcome: " + this.numRounds + " * " + this.totalHitPoints + " = " + (this.numRounds * this.totalHitPoints));
            System.out.println("Required elf power was: " + this.elfPower);
        }
    }

    static String readFile(String filename) throws Exception {
        BufferedReader br = new BufferedReader(new FileReader(new File(filename)));

        String st;
        String result = "";
        int line = 0;
        while ((st = br.readLine()) != null) {
            result += st;
            result += "\n";
        }

        return result;
    }

    static Sim fromString(String input) {
        Sim sim = new Sim();

        String[] split = input.split("\n");

        for (int line = 0; line < split.length; line++) {
            String st = split[line];
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
        }

        return sim;
    }

    static Result rullAllAndCheckNeededElfPower(boolean debug, String input) {
        int elfPower = 3;
        boolean elvesDied = true;

        Result result = null;
        while (elvesDied) {
            if (debug) System.out.println("Trying elf power: " + elfPower);
            Sim sim = Sim.fromString(input);

            try {
                result = sim.runFull(debug, elfPower);
                elvesDied = false;
                result.elfPower = elfPower;

                return result;
            } catch (ElfDiedException e) {
                elfPower++;
            }
        }
        return result;
    }

    Result runFull(boolean debug, int elfPower) throws ElfDiedException {
        if (debug) {
            System.out.println("Initial");
            System.out.println(this);
        }

        setElfPower(elfPower);

        int i = 0;
        while (!this.didAnybodyWin().isPresent()) {
            Optional<PlayerType> didAnybodyWin = this.runRound(debug);

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

        if (debug) result.print();

        return result;

    }

    private void setElfPower(int elfPower) {
        this.players.stream()
                .filter(p -> p.type == PlayerType.ELF)
                .forEach(p -> p.attackPower = elfPower);
    }

    Optional<PlayerType> runRound(boolean debug) throws ElfDiedException {
        this.sortPlayers();

        for (Player player : players) {
            if (!player.alive) continue;

            if (!didAnybodyWin().isPresent()) {
                runRoundForPlayer(player, debug);
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

    Optional<PlayerType> runRoundForPlayer(Player player, boolean debug) throws ElfDiedException {
        if (!player.alive) return Optional.empty();
        if (didAnybodyWin().isPresent()) return didAnybodyWin();

        // Move
        Coord movePos = this.selectFirstStep(player);
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

                if (enemy.type == PlayerType.ELF) {
                    if (debug) System.out.println("Elf died, stopping");
                    throw new ElfDiedException();
                }
            }
        }

        return didAnybodyWin();
    }

    class ElfDiedException extends Exception {

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

    Map<Coord, Route> getMapFrom(Coord pos) {
        Map<Coord, Route> routes = new HashMap<>();

        Coord start = pos;

        Deque<Coord> positions = new ArrayDeque<>();
        positions.add(start);

        routes.put(start, new Route(start));

        while (!positions.isEmpty()) {
            Coord currentPos = positions.removeFirst();
            Route currentRoute = routes.get(currentPos);

            for (Coord potentialMove : currentPos.coordsInRange()) {
                if (this.walls.contains(potentialMove)) {
                    continue;
                }

                if (this.getPlayerAt(potentialMove) != null) {
                    continue;
                }

                Route routeTo = currentRoute.createFromAndAdd(potentialMove);

                if (routes.containsKey(potentialMove) && routes.get(potentialMove).length() <= routeTo.length()) {
                    continue;
                }

                routes.put(potentialMove, routeTo);
                positions.addLast(potentialMove);
            }

        }
        return routes;
    }

    Coord selectTargetPoint(Player player, Map<Coord, Route> routeMap) {
        if (!player.alive) return null;
        if (this.playerInRangeOfEnemy(player)) return null;

        List<Route> routesToTargets = this.pointsInRangeOfEnemy(player)
                .map(c -> routeMap.get(c))
                .filter(r -> r != null)
                .collect(Collectors.toList());

        Optional<Integer> shortestLengthOpt = routesToTargets.stream()
                .map(r -> r.length())
                .sorted()
                .findFirst();
        
        if (!shortestLengthOpt.isPresent()) return null;

        Integer shortestLength = shortestLengthOpt.get();

        List<Route> routesToShortestTargets = routesToTargets.stream()
                .filter(r -> r.length() == shortestLength)
                .sorted((r1, r2) -> {
                    Coord target1 = r1.getLastStep();
                    Coord target2 = r2.getLastStep();

                    return target1.compareTo(target2);
                })
                .collect(Collectors.toList());

        return routesToShortestTargets.get(0).getFirstStep();
    }

    Coord selectFirstStep(Player player) {
        if (!player.alive) return null;
        if (this.playerInRangeOfEnemy(player)) return null;

        Map<Coord, Route> targetMap = this.getMapFrom(player.pos);
        Coord target = this.selectTargetPoint(player, targetMap);

        if (target == null) return null;
        else {
            Map<Coord, Route> mapBack = this.getMapFrom(target);

            List<Coord> potentialMoves = player.pos.coordsInRange();

            List<Route> routesBack = potentialMoves.stream()
                    .filter(c -> !this.walls.contains(c))
                    .filter(c -> this.getPlayerAt(c) == null)
                    .map(c -> mapBack.get(c))
                    .filter(r -> r != null)
                    .collect(Collectors.toList());

            int minLength = Integer.MAX_VALUE;
            HashSet<Route> shortestRoutes = new HashSet<Route>();

            for (Route route : routesBack) {
                if (route.length() < minLength) {
                    shortestRoutes.clear();
                    shortestRoutes.add(route);
                    minLength = route.length();
                } else if (route.length() == minLength) {
                    shortestRoutes.add(route);
                }
            }

            Optional<Coord> opt = shortestRoutes.stream()
                    .map(r -> r.getLastStep())
                    .sorted()
                    .findFirst();

            if (opt.isPresent()) return opt.get();
            else return null;
        }
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
