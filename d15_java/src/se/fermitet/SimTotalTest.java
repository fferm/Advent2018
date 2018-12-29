package se.fermitet;

import org.junit.jupiter.api.Test;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.core.Is.is;

public class SimTotalTest {
    @Test
    public void test1() throws Exception {
        String input = Sim.readFile("inputs/1.txt");
        Sim.Result result = Sim.rullAllAndCheckNeededElfPower(false, input);

        assertThat(result.winner, is(PlayerType.ELF));
        assertThat(result.numRounds, is(29));
        assertThat(result.totalHitPoints, is(172));
        assertThat(result.elfPower, is(15));
    }

//    @Test
//    public void test2() throws Exception {
//        String input = Sim.readFile("inputs/2.txt");
//        Sim.Result result = Sim.rullAllAndCheckNeededElfPower(false, input);
//
//        assertThat(result.winner, is(PlayerType.ELF));
//        assertThat(result.numRounds, is(37));
//        assertThat(result.totalHitPoints, is(982));
//    }

//    @Test
//    public void test3() throws Exception {
//        String input = Sim.readFile("inputs/3.txt");
//        Sim.Result result = Sim.rullAllAndCheckNeededElfPower(true, input);
//
//        assertThat(result.winner, is(PlayerType.ELF));
//        assertThat(result.numRounds, is(46));
//        assertThat(result.totalHitPoints, is(859));
//    }

//    @Test
//    public void test4() throws Exception {
//        String input = Sim.readFile("inputs/4.txt");
//        Sim.Result result = Sim.rullAllAndCheckNeededElfPower(true, input);
//
//        assertThat(result.winner, is(PlayerType.GOBLIN));
//        assertThat(result.numRounds, is(35));
//        assertThat(result.totalHitPoints, is(793));
//    }

//    @Test
//    public void test5() throws Exception {
//        String input = Sim.readFile("inputs/5.txt");
//        Sim.Result result = Sim.rullAllAndCheckNeededElfPower(true, input);
//
//        assertThat(result.winner, is(PlayerType.GOBLIN));
//        assertThat(result.numRounds, is(54));
//        assertThat(result.totalHitPoints, is(536));
//    }

//    @Test
//    public void test6() throws Exception {
//        String input = Sim.readFile("inputs/6.txt");
//        Sim.Result result = Sim.rullAllAndCheckNeededElfPower(true, input);
//
//        assertThat(result.winner, is(PlayerType.GOBLIN));
//        assertThat(result.numRounds, is(20));
//        assertThat(result.totalHitPoints, is(937));
//    }

//    @Test
//    public void moveRight() throws Exception {
//        String input =
//                "#######" + "\n" +
//                "#.E..G#" + "\n" +
//                "#.#####" + "\n" +
//                "#G#####" + "\n" +
//                "#######";
//
//        Sim sim = Sim.fromString(input);
//        Sim.Result result = sim.runFull(false, 3);
//
//        assertThat(result.winner, is(PlayerType.GOBLIN));
//        assertThat(result.numRounds, is(34));
//        assertThat(result.totalHitPoints, is(301));
//    }

}

