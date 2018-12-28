package se.fermitet;

import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.core.Is.*;
import static org.hamcrest.core.IsCollectionContaining.*;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertNull;

public class SimTest {


    @Test
    public void pointsInRangeOfEnemy() throws Exception {
        String input =  "#######" + "\n" +
                        "#E..G.#" + "\n" +
                        "#...#.#" + "\n" +
                        "#.G.#G#" + "\n" +
                        "#######" + "\n";

        Sim sim = Sim.fromString(input);
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

    @Test
    public void pointsInRangeOfEnemy_notAffectedByDeadEnemies() throws Exception {
        String input =  "#######" + "\n" +
                "#E..G.#" + "\n" +
                "#...#.#" + "\n" +
                "#.G.#G#" + "\n" +
                "#######" + "\n";

        Sim sim = Sim.fromString(input);

        Player deadGoblin = new Player(new Coord(1, 2), PlayerType.GOBLIN);
        deadGoblin.alive = false;
        sim.players.add(deadGoblin);

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

    @Test
    public void map() {
        String input =
                "#######" + "\n" +
                "#E..G.#" + "\n" +
                "#...#.#" + "\n" +
                "#.G.#G#" + "\n" +
                "#######" + "\n";

        Sim sim = Sim.fromString(input);
        Player elf = sim.getPlayerAt(new Coord(1, 1));

        Map<Coord, Route> map = sim.getMapFrom(elf.pos);

        Route r11 = map.get(new Coord(1, 1));
        assertNotNull(r11);
        assertThat(r11.length(), is(0));

        Route r12 = map.get(new Coord(1, 2));
        assertNotNull(r12);
        assertThat(r12.length(), is(1));
        assertThat(r12.steps.get(0), is(new Coord(1, 1)));
        assertThat(r12.steps.get(1), is(new Coord(1, 2)));

        Route r21 = map.get(new Coord(2, 1));
        assertNotNull(r21);
        assertThat(r21.length(), is(1));
        assertThat(r21.steps.get(0), is(new Coord(1, 1)));
        assertThat(r21.steps.get(1), is(new Coord(2, 1)));

        Route r22 = map.get(new Coord(2, 2));
        assertNotNull(r22);
        assertThat(r22.length(), is(2));
        assertThat(r22.steps.get(0), is(new Coord(1, 1)));
        assertThat(r22.steps.get(1), is(new Coord(2, 1)));
        assertThat(r22.steps.get(2), is(new Coord(2, 2)));

        Route r31 = map.get(new Coord(3, 1));
        assertNotNull(r31);
        assertThat(r31.length(), is(2));
        assertThat(r31.steps.get(0), is(new Coord(1, 1)));
        assertThat(r31.steps.get(1), is(new Coord(2, 1)));
        assertThat(r31.steps.get(2), is(new Coord(3, 1)));

        Route r13 = map.get(new Coord(1, 3));
        assertNotNull(r13);
        assertThat(r13.length(), is(2));
        assertThat(r13.steps.get(0), is(new Coord(1, 1)));
        assertThat(r13.steps.get(1), is(new Coord(1, 2)));
        assertThat(r13.steps.get(2), is(new Coord(1, 3)));

        Route r32 = map.get(new Coord(3, 2));
        assertNotNull(r32);
        assertThat(r32.length(), is(3));
        assertThat(r32.steps.get(0), is(new Coord(1, 1)));
        assertThat(r32.steps.get(1), is(new Coord(2, 1)));
        assertThat(r32.steps.get(2), is(new Coord(3, 1)));
        assertThat(r32.steps.get(3), is(new Coord(3, 2)));

        Route r33 = map.get(new Coord(3, 3));
        assertNotNull(r33);
        assertThat(r33.length(), is(4));
        assertThat(r33.steps.get(0), is(new Coord(1, 1)));
        assertThat(r33.steps.get(1), is(new Coord(2, 1)));
        assertThat(r33.steps.get(2), is(new Coord(3, 1)));
        assertThat(r33.steps.get(3), is(new Coord(3, 2)));
        assertThat(r33.steps.get(4), is(new Coord(3, 3)));

        assertThat(map.size(), is(8));
    }

    @Test
    public void selectTargetPoint_no_route() {
        String input =
        "#######" + "\n" +
        "#G....#" + "\n" +
        "#.G...#" + "\n" +
        "#.#.#G#" + "\n" +
        "#...#E#" + "\n" +
        "#..G..#" + "\n" +
        "#######";

        Sim sim = Sim.fromString(input);
        Player goblin = sim.getPlayerAt(new Coord(1, 1));

        Map<Coord, Route> map = sim.getMapFrom(goblin.pos);
        assertNull(sim.selectTargetPoint(goblin, map));
    }


    @Test
    public void selectFirstStep() {
        String input =
                "#######" + "\n" +
                "#E..G.#" + "\n" +
                "#...#.#" + "\n" +
                "#.G.#G#" + "\n" +
                "#######" + "\n";

        Sim sim = Sim.fromString(input);
        Player elf = sim.getPlayerAt(new Coord(1, 1));

        assertThat(sim.selectFirstStep(elf), is(new Coord(2, 1)));

    }

    @Test
    public void selectFirstStep_no_route() {
        String input =
                "#######" + "\n" +
                        "#G....#" + "\n" +
                        "#.G...#" + "\n" +
                        "#.#.#G#" + "\n" +
                        "#...#E#" + "\n" +
                        "#..G..#" + "\n" +
                        "#######";

        Sim sim = Sim.fromString(input);
        Player goblin = sim.getPlayerAt(new Coord(1, 1));

        assertNull(sim.selectFirstStep(goblin));
    }

/*    @Test
    public void selectFirstStep_shortestPathBack() {
        String input =
            "#######" + "\n" +
            "#...E.#" + "\n" +
            "#...#.#" + "\n" +
            "#...#.#" + "\n" +
            "#.....#" + "\n" +
            "#...G.#" + "\n" +
            "#######";

        Sim sim = Sim.fromString(input);
        Player player = sim.getPlayerAt(new Coord(4, 1));

        System.out.println(sim.selectFirstStep(player));

        assertNull(sim.selectFirstStep(player));

    }*/

}