package se.fermitet;

import java.util.ArrayList;
import java.util.List;

class Route {
    List<Coord> steps = new ArrayList<>();
    Coord enemyPosition;
    Coord ownEndPosition;

    private Route() {
        super();
    }

    Route(Coord start) {
        this();
        this.addCoord(start);
    }

    Route createFromAndAdd(Coord potentialMove) {
        Route ret = new Route();

        for (Coord c : this.steps) {
            ret.addCoord(c);
        }
        ret.addCoord(potentialMove);

        return ret;
    }

    private void addCoord(Coord c) {
        this.steps.add(c);
    }

    public int length() {
        return steps.size() - 1;
    }

    Coord getLastStep() {
        return this.steps.get(this.steps.size() - 1);
    }

    Coord getFirstStep() {
        return this.steps.get(1);
    }

    @Override
    public String toString() {
        String ret = "[";

        for (Coord c : this.steps) {
            ret += c.toString() + ", ";
        }

        if (this.enemyPosition != null) {
            ret += "ENEMY: " + enemyPosition;
        }

        ret += "]";

        return ret;
    }

}
