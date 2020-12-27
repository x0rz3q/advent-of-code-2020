public class Main {
    public Main() { }

    public long crack(long loopSize, long subject) {
        long result = 1;

        for (int i = 0; i < loopSize; i++) {
            result = result * subject;
            result = result % 20201227;
        }

        return result;
    }

    public void run() {
        long publicDoor = 4126658;
        long secretCard = 1568743;

        long key = crack(secretCard, publicDoor);
        System.out.println("Silver: " + key);
    }

    public static void main(String[] args) {
        (new Main()).run();
    }
}
