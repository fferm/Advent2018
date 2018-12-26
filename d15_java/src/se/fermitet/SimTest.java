package se.fermitet;

import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.stream.Collectors;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.core.Is.*;
import static org.hamcrest.core.IsCollectionContaining.*;

public class SimTest {
    @Test
    public void test1() throws Exception {
        Sim sim = Sim.fromFile("inputs/1.txt");
        Sim.Result result = sim.runFull(false);

        assertThat(result.winner, is(PlayerType.GOBLIN));
        assertThat(result.numRounds, is(47));
        assertThat(result.totalHitPoints, is(590));
    }

    @Test
    public void test2() throws Exception {
        Sim sim = Sim.fromFile("inputs/2.txt");
        Sim.Result result = sim.runFull(false);

        assertThat(result.winner, is(PlayerType.ELF));
        assertThat(result.numRounds, is(37));
        assertThat(result.totalHitPoints, is(982));
    }

    @Test
    public void test3() throws Exception {
        Sim sim = Sim.fromFile("inputs/3.txt");
        Sim.Result result = sim.runFull(false);

        assertThat(result.winner, is(PlayerType.ELF));
        assertThat(result.numRounds, is(46));
        assertThat(result.totalHitPoints, is(859));
    }

    @Test
    public void test4() throws Exception {
        Sim sim = Sim.fromFile("inputs/4.txt");
        Sim.Result result = sim.runFull(false);

        assertThat(result.winner, is(PlayerType.GOBLIN));
        assertThat(result.numRounds, is(35));
        assertThat(result.totalHitPoints, is(793));
    }

    @Test
    public void test5() throws Exception {
        Sim sim = Sim.fromFile("inputs/5.txt");
        Sim.Result result = sim.runFull(false);

        assertThat(result.winner, is(PlayerType.GOBLIN));
        assertThat(result.numRounds, is(54));
        assertThat(result.totalHitPoints, is(536));
    }

    @Test
    public void test6() throws Exception {
        Sim sim = Sim.fromFile("inputs/6.txt");
        Sim.Result result = sim.runFull(false);

        assertThat(result.winner, is(PlayerType.GOBLIN));
        assertThat(result.numRounds, is(20));
        assertThat(result.totalHitPoints, is(937));
    }

    @Test
    public void pointsInRange() throws Exception {
        Sim sim = Sim.fromFile("inputs/move.txt");
        Player elf = sim.getPlayerAt(new Coord(1, 1));

        List<Coord> pointsInRange = sim.pointsInRangeOfEnemy(elf).collect(Collectors.toList());

        assertThat(pointsInRange.size(), is(6));
        assertThat(pointsInRange, hasItem(new Coord(3, 1)));
        assertThat(pointsInRange, hasItem(new Coord(5, 1)));
        assertThat(pointsInRange, hasItem(new Coord(2, 2)));
        assertThat(pointsInRange, hasItem(new Coord(5, 2)));
        assertThat(pointsInRange, hasItem(new Coord(1, 3)));
        assertThat(pointsInRange, hasItem(new Coord(3, 3)));
    }
}