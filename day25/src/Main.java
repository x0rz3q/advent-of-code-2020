public class Main {

    public Main() {

    }

    static long modInverse(long a, long m)
    {
        a = a % m;
        for (long x = 1; x < m; x++)
            if ((a * x) % m == 1)
                return x;
        return 1;
    }

    public long crack(long loopSize, long subject) {
        long result = 1;

        // subject * ? % loopsize = 1

        for (int i = 0; i < loopSize; i++) {
            result = result * subject;
            result = result % 20201227;
        }

        return result;
    }

    public long getSecret(long target) {
        long result = 0;
        long secretLoopSize = 0;

        System.out.println(modInverse(target, 20201227));


//        while (result != target) {
//            secretLoopSize++;
//            result = crack(secretLoopSize, 7);
//        }

        return secretLoopSize;
    }

    public void run() {
        long publicCard = 10604480;
        long publicDoor = 4126658;
        long secretCard = 1568743;
        long secretDoor = 11;

        long key = crack(secretCard, publicDoor);
        System.out.println(key);
//        System.out.println(privateDoor);
    }

    public static void main(String[] args) {
        (new Main()).run();
    }
}
