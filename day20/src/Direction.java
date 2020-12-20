public enum Direction {
    TOP(0),
    BOTTOM(1),
    LEFT(2),
    RIGHT(3),
    NONE(1000);

    private final int value;

    private Direction(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }
}
