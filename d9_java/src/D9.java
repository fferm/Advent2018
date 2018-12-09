import java.util.HashMap;
import java.util.Map;

public class D9 {
    static int START = 0;
    public static void main(String[] args) {
        D9 d9 = new D9();
        d9.run();
    }

    private void run() {
        int numPlayers = 424;
        int numSteps = 71482 * 100;
//        int numPlayers = 30;
//        int numSteps = 5807;

        CircleElement current = new CircleElement(START);
        current.cw = current;
        current.ccw = current;

        int currentPlayer = 0;
        int newMarble = 1;

        Map<Integer, Long> scores = new HashMap<>();

        while (newMarble <= numSteps) {
            if (newMarble < 100)
                print(current, currentPlayer);

            if (newMarble % 23 == 0) {
                CircleElement removePoint = current.ccw.ccw.ccw.ccw.ccw.ccw.ccw;

                int removeScore = removePoint.value;
                addScoreToPlayer(scores, currentPlayer, removeScore);
                addScoreToPlayer(scores, currentPlayer, newMarble);

                current = removePoint.remove();

            } else {
                CircleElement insertPoint = current.cw;
                current = insertPoint.insert(newMarble);
            }

            currentPlayer = (currentPlayer % numPlayers) + 1;
            newMarble++;

            if (newMarble % 100000 == 0) {
                System.out.println("playing...   newMarble: " + newMarble + "   target: " + numSteps);

                int max_player = 0;
                long max_score = 0;
                for (int player: scores.keySet()) {
                    long score = scores.get(player);
                    if (score > max_score) {
                        max_player = player;
                        max_score = score;
                    }
                }
                System.out.println("Player " + max_player + " has a score of " + max_score);
            }

        }

        int max_player = 0;
        long max_score = 0;
        for (int player: scores.keySet()) {
            long score = scores.get(player);
            if (score > max_score) {
                max_player = player;
                max_score = score;
            }
        }
        System.out.println("Player " + max_player + " wins with  a score of " + max_score);

    }

    void addScoreToPlayer(Map<Integer, Long> scores, int currentPlayer, int scoreToAdd) {
        if (scores.containsKey(currentPlayer)) {
            scores.put(currentPlayer, scores.get(currentPlayer) + scoreToAdd);
        } else {
            scores.put(currentPlayer, (long) scoreToAdd);
        }
    }


    private void print(CircleElement element, int currentPlayer) {
        CircleElement current = element;

        while(current.value != START) {
            current = current.cw;
        }

        System.out.print("[" + currentPlayer + "] ");

        do {
            if (current == element) {
                System.out.print("(" + current.value + ")");
            } else {
                System.out.print(" " + current.value + " ");
            }
            current = current.cw;
        } while (current.value != START);
        System.out.println();
    }

}

class CircleElement {
    int value;
    CircleElement cw;
    CircleElement ccw;

    CircleElement(int value) {
        this(value, null, null);
    }

    CircleElement(int value, CircleElement cw, CircleElement ccw) {
        super();
        this.value = value;
        this.cw = cw;
        this.ccw = ccw;
    }

    CircleElement insert(int value) {
        CircleElement cw = this.cw;

        CircleElement toInsert = new CircleElement(value, this.cw, this);

        this.cw = toInsert;
        cw.ccw = toInsert;

        return toInsert;
    }

    CircleElement remove() {
        CircleElement ccw = this.ccw;
        CircleElement cw = this.cw;

        ccw.cw = cw;
        cw.ccw = ccw;

        return cw;
    }
}
