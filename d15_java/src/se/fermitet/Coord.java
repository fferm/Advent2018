package se.fermitet;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

class Coord implements Comparable<Coord> {
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

    public List<Coord> coordsInRange() {
        List<Coord> ret = new ArrayList<>();

        ret.add(new Coord(this.x, this.y - 1));
        ret.add(new Coord(this.x - 1, this.y));
        ret.add(new Coord(this.x + 1, this.y));
        ret.add(new Coord(this.x, this.y + 1));

        return ret;
    }

    @Override
    public int compareTo(Coord o) {
        int yCmp = this.y - o.y;

        if (yCmp != 0) {
            return yCmp;
        } else {
            return this.x - o.x;
        }
    }
}
