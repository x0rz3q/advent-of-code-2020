import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;
import java.util.Scanner;

public class Main {
    public Main() { }

    public long silver(List<Tile> tiles) {
        long product = 1;

        for (Tile first : tiles) {
            int count = 0;

            for (Tile second : tiles) {
                if (second.equals(first)) {
                    continue;
                }

                if (first.wouldTouch(second)) {
                    first.registerTouch(second);

                    count++;
                }
            }

            if (count == 2) {
                first.setIsCorner(true);

                product *= first.getId();
                System.out.println("Corner found: " + first.getId());
            }
        }

        return product;
    }

    public List<List<Character>> flip(List<List<Character>> grid) {
        List<List<Character>> flipped = new ArrayList<>();

        for (List<Character> row : grid) {
            List<Character> newRow = new ArrayList<>(row);
            Collections.reverse(newRow);
            flipped.add(newRow);
        }

        return flipped;
    }

    public List<List<Character>> rotate(List<List<Character>> grid) {
        List<List<Character>> rotated = new ArrayList<>();

        for (int i = 0; i < grid.get(0).size(); i++) {
            List<Character> column = new ArrayList<>();

            for (int j = 0; j < grid.size(); j++) {
                column.add(grid.get(j).get(i));
            }

            Collections.reverse(column);
            rotated.add(column);
        }

        return rotated;
    }

    public boolean hasMonster(List<List<Character>> grid, int x, int y, boolean write) {
        int[][] offsets = {
                {0, 0},
                {0, 1},
                {1, 1},
                {-1, 1},
                {-6, 1},
                {-7, 1},
                {-12, 1},
                {-13, 1},
                {-18, 1},
                {-2, 2},
                {-5, 2},
                {-8, 2},
                {-11, 2},
                {-14, 2},
                {-17, 2}
        };

        for (int i = 0; i < offsets.length; i++) {
            int[] offset = offsets[i];
            int xOffset = x + offset[0];
            int yOffset = y + offset[1];

            if (yOffset >= grid.size() || yOffset < 0) {
                return false;
            }

            if (xOffset >= grid.size() || xOffset < 0) {
                return false;
            }

            if (grid.get(yOffset).get(xOffset) != '#') {
                return false;
            }
        }

        if (write) {
            for (int i = 0; i < offsets.length; i++) {
                int[] offset = offsets[i];
                int xOffset = x + offset[0];
                int yOffset = y + offset[1];

                grid.get(yOffset).set(xOffset, 'O');
            }
        }

        return true;
    }

    public void printGrid(List<List<Character>> grid) {
        for (List<Character> row : grid) {
            StringBuilder builder = new StringBuilder();

            for (Character c : row) {
                builder.append(c);
            }

            System.out.println(builder.toString());
        }
    }

    public long gold(List<Tile> tiles) {
        // First setup the corners, this is the easiest to do because
        Tile corner = null;

        for (Tile tile : tiles) {
            if (tile.isCorner()) {
                corner = tile;
                break;
            }
        }

        assert corner != null;

        for (Tile neighbour : corner.getTouching()) {
            if (corner.touches(neighbour)) {
                corner.register(neighbour);
                System.out.println("Touched!");
            } else {
                neighbour.flip();

                if (corner.touches(neighbour)) {
                    corner.register(neighbour);
                    System.out.println("Touched!");
                } else {
                    throw new IllegalStateException("Did not touch :(");
                }
            }
        }

        // Find the direction to walk in :3
        List<Direction> directions = new ArrayList<>();
        Queue<Tile> queue = new LinkedList<Tile>();
        List<Tile> seen = new ArrayList<>();
        seen.add(corner);

        if (corner.getLeft() != null) {
            directions.add(Direction.LEFT);
            queue.add(corner.getLeft());
            seen.add(corner.getLeft());
        }

        if (corner.getRight() != null) {
            directions.add(Direction.RIGHT);
            queue.add(corner.getRight());
            seen.add(corner.getRight());
        }

        if (corner.getBottom() != null) {
            directions.add(Direction.BOTTOM);
            queue.add(corner.getBottom());
            seen.add(corner.getBottom());
        }

        if (corner.getTop() != null) {
            directions.add(Direction.TOP);
            queue.add(corner.getTop());
            seen.add(corner.getTop());
        }

        while (!queue.isEmpty()) {
            Tile tile = queue.poll();

            System.out.println("Starting with " + tile.getId());

            for (Tile neighbour : tile.getTouching()) {
                if (tile.touches(neighbour)) {
                    tile.register(neighbour);
                    System.out.println("Touched!");
                } else {
                    neighbour.flip();

                    if (tile.touches(neighbour)) {
                        tile.register(neighbour);
                        System.out.println("Touched!");
                    } else {
                        throw new IllegalStateException("Did not touch :( " + neighbour.getId());
                    }
                }

                if (!seen.contains(neighbour)) {
                    queue.add(neighbour);
                    seen.add(neighbour);
                }
            }
        }

        // find top right
        for (Tile tile : tiles) {
            if (tile.getTop() == null && tile.getLeft() == null) {
                corner = tile;
                break;
            }
        }

        queue = new LinkedList<>();
        queue.add(corner);

        List<List<Character>> grid = new ArrayList<>();

        while (!queue.isEmpty()) {
            Tile tile = queue.poll();

            for (int i = 1; i < tile.getGrid().size() - 1; i++) {
                List<Character> row = new ArrayList<>();

                Tile right = tile;

                while (right != null) {
                    row.addAll(right.getBorderLessRow(i));
                    right = right.getRight();
                }

                grid.add(row);
            }

            if (tile.getBottom() != null) {
                queue.add(tile.getBottom());
            }
        }

        // Time to find the monster!

        for (int k = 0; k < 3; k++) {
            for (int i = 0; i < 4; i++) {
                System.out.println("Starting rotation");
                for (int y = 0; y < grid.size(); y++) {
                    for (int x = 0; x < grid.size(); x++) {
                        if (hasMonster(grid, x, y, true)) {
                            System.out.println("Had monster");
                        }
                    }
                }

                grid = rotate(grid);
            }

            grid = flip(grid);
        }

        long count = 0;
        for (List<Character> row : grid) {
            for (Character c : row) {
                if (c == '#') {
                    count++;
                }
            }
        }

        return count;
    }

    public void run() throws FileNotFoundException {
        File file = new File("input");
        Scanner scanner = new Scanner(file);

        List<Tile> tiles = new ArrayList<>();
        Tile tile = new Tile();

        while (scanner.hasNextLine()) {
            String line = scanner.nextLine();

            if (line.contains("Tile")) {
                tile.setId(Integer.parseInt(line.split("Tile ")[1].split(":")[0]));
            } else if (line.contains("#") || line.contains(".")) {
                tile.addGridLine(line);
            } else {
                tile.parseBorders();

                tiles.add(tile);
                tile = new Tile();
            }
        }

        System.out.println("Silver: " + silver(tiles));
        System.out.println("Gold: " + gold(tiles));
    }

    public static void main(String[] args) throws FileNotFoundException {
        (new Main()).run();
    }
}
