package se.fermitet;

public class Main {

    public static void main(String[] args) throws Exception{
        new Main().run();
    }

    private void run() throws Exception {
        boolean smallInput = false;
        String filename;
        if (smallInput) {
            filename = "inputs/1.txt";
//            filename = "input_small.txt";
        } else {
            filename = "input.txt";
//            filename = "input_debug.txt";
        }

        Sim sim = Sim.fromFile(filename);

        System.out.println(sim);

        Sim.Result result = sim.runFull(true);

        System.out.println("Combat ends after " + result.numRounds + " full rounds");
        System.out.println(result.winner + " wins with " + result.totalHitPoints + " left");
        System.out.println("Outcome: " + result.numRounds + " * " + result.totalHitPoints + " = " + (result.numRounds * result.totalHitPoints));

    }
}





