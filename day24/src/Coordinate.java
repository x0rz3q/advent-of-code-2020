import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public class Coordinate {
    int x;
    int y;

    public Coordinate(int x, int y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Coordinate that = (Coordinate) o;
        return x == that.x &&
                y == that.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }

    public Coordinate move(String direction) {
        // Do some mapping so that we can use coordinates. We just assume we are at the top of the
        // tile. Then we can make easy decisions.
        switch (direction) {
            case "se":
                return new Coordinate(this.x + 1, this.y - 1);
            case "nw":
                return new Coordinate(this.x - 1, this.y + 1);
            case "e":
                return new Coordinate(this.x + 1, this.y);
            case "sw":
                return new Coordinate(this.x, this.y - 1);
            case "w":
                return new Coordinate(this.x - 1, this.y);
            case "ne":
                return new Coordinate(this.x, this.y + 1);
            default:
                System.err.println("Not implemented");
                break;
        }

        return null;
    }

    public List<Coordinate> getAdjacent() {
        List<Coordinate> coordinates = new ArrayList<>();

        coordinates.add(this.move("se"));
        coordinates.add(this.move("nw"));
        coordinates.add(this.move("e"));
        coordinates.add(this.move("sw"));
        coordinates.add(this.move("w"));
        coordinates.add(this.move("ne"));

        return coordinates;
    }
}
