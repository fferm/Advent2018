package se.fermitet;

class Player implements Comparable<Player>{
    Coord pos;
    PlayerType type;
    int hitPoints;
    int attackPower;
    boolean alive = true;

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

    @Override
    public int compareTo(Player o) {
        return this.pos.compareTo(o.pos);
    }
}
