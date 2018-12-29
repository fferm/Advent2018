package se.fermitet;

public class Main {

    public static void main(String[] args) throws Exception{
        new Main().run();
    }

    private void run() throws Exception {
        boolean smallInput = false;
        String filename;
        if (smallInput) {
            filename = "inputs/larger.txt";
//            filename = "input_small.txt";
        } else {
            filename = "input.txt";
//            filename = "input_debug.txt";
        }

        Sim sim = Sim.fromFile(filename);

        System.out.println(sim);

        Sim.Result result = sim.runFull(true);

        result.print();

    }
}





