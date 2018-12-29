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

        String input = Sim.readFile(filename);
        Sim.Result result = Sim.rullAllAndCheckNeededElfPower(true, input);

        result.print();
    }
}





