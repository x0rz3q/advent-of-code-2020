import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

public class Tile {
    private int id;
    private List<List<Character>> grid;
    private List<String> borders;
    private boolean isCorner;

    private Tile left;
    private Tile right;
    private Tile bottom;
    private Tile top;

    private List<Tile> touching;

    public Tile() {
        this.grid = new ArrayList<>();
        this.borders = new ArrayList<>();
        this.id = 0;
        this.isCorner = false;

        this.touching = new ArrayList<>();
    }

    public void registerTouch(Tile other) {
        this.touching.add(other);
    }

    public void setId(int id) {
        this.id = id;
    }

    public int getId() {
        return this.id;
    }

    public void setLeft(Tile left) {
        this.left = left;
    }

    public void setRight(Tile right) {
        this.right = right;
    }

    public void setTop(Tile top) {
        this.top = top;
    }

    public void setBottom(Tile bottom) {
        this.bottom = bottom;
    }

    public Tile getBottom() {
        return bottom;
    }

    public Tile getLeft() {
        return left;
    }

    public Tile getRight() {
        return right;
    }

    public Tile getTop() {
        return top;
    }

    public void setIsCorner(boolean isCorner) {
        this.isCorner = isCorner;
    }

    public boolean isCorner() {
        return isCorner;
    }

    public List<String> getBorders() {
        return this.borders;
    }

    public boolean wouldTouch(Tile other) {
        for (String border : this.getBorders()) {
            String reversed = (new StringBuffer()).append(border).reverse().toString();

            if (other.getBorders().contains(border) || other.getBorders().contains(reversed)) {
                return true;
            }
        }

        return false;
    }

    public Direction getTouchLocation(Tile other) {
        if (other.getBorders().get(Direction.TOP.getValue()).equals(this.getBorders().get(Direction.BOTTOM.getValue()))) {
            return Direction.BOTTOM;
        } else if (other.getBorders().get(Direction.BOTTOM.getValue()).equals(this.getBorders().get(Direction.TOP.getValue()))) {
            return Direction.TOP;
        } else if (other.getBorders().get(Direction.LEFT.getValue()).equals(this.getBorders().get(Direction.RIGHT.getValue()))) {
            return Direction.RIGHT;
        } else if (other.getBorders().get(Direction.RIGHT.getValue()).equals(this.getBorders().get(Direction.LEFT.getValue()))) {
            return Direction.LEFT;
        }

        return Direction.NONE;
    }

    public boolean touches(Tile other) {
        for (int i = 0; i < 4; i++) {
            other.rotate();

            if (this.getTouchLocation(other) != Direction.NONE) {
                return true;
            }
        }

        return false;
    }

    public List<Tile> getTouching() {
        return this.touching;
    }

    public List<List<Character>> getGrid() {
        return grid;
    }

    public void register(Tile other) {
        Direction location = this.getTouchLocation(other);

        if (location == Direction.NONE) {
            System.out.println("" + this.id + " -> " + other.getId());
            throw new IllegalArgumentException("Not touching!");
        }

        switch (location) {
            case TOP:
                this.setTop(other);
                other.setBottom(this);
                break;
            case BOTTOM:
                this.setBottom(other);
                other.setTop(this);
                break;
            case LEFT:
                this.setLeft(other);
                other.setRight(this);
                break;
            case RIGHT:
                this.setRight(other);
                other.setLeft(this);
                break;
        }
    }

    public void addGridLine(String line) {
        grid.add(line.chars().mapToObj(c -> (char) c).collect(Collectors.toList()));
    }

    public void parseBorders() {
        this.borders = new ArrayList<>();

        this.borders.add(this.grid.get(0).stream().map(String::valueOf).collect(Collectors.joining()));
        this.borders.add(this.grid.get(this.grid.size() - 1).stream().map(String::valueOf).collect(Collectors.joining()));

        StringBuilder left = new StringBuilder();
        StringBuilder right = new StringBuilder();

        for (List<Character> row : grid) {
            left.append(row.get(0));
            right.append(row.get(grid.size() - 1));
        }

        this.borders.add(left.toString());
        this.borders.add(right.toString());
    }

    /**
     * Rotate 90 degrees clockwise, -> that direction
     */
    public void rotate() {
        List<List<Character>> rotated = new ArrayList<>();

        for (int i = 0; i < this.grid.get(0).size(); i++) {
            List<Character> column = new ArrayList<>();

            for (int j = 0; j < this.grid.size(); j++) {
                column.add(this.grid.get(j).get(i));
            }

            Collections.reverse(column);
            rotated.add(column);
        }

        this.grid = rotated;

        this.parseBorders();
    }

    public String toString() {
        StringBuilder builder = new StringBuilder();

        for (List<Character> row : this.grid) {
            builder.append(row.stream().map(String::valueOf).collect(Collectors.joining()));
            builder.append("\n");
        }

        return builder.toString();
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Tile tile = (Tile) o;
        return id == tile.id;
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }

    public void flip() {
        List<List<Character>> flipped = new ArrayList<>();

        for (List<Character> row : this.grid) {
            List<Character> newRow = new ArrayList<>(row);
            Collections.reverse(newRow);
            flipped.add(newRow);
        }

        this.grid = flipped;
    }

    public List<Character> getBorderLessRow(int row) {
        if (row == 0 || row == this.grid.size() - 1) {
            throw new IllegalArgumentException("No borders pls");
        }

        return this.grid.get(row).subList(1, this.grid.size() - 1);
    }
}
