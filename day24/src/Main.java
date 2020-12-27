import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
    public HashMap<Coordinate, Boolean> run() throws FileNotFoundException {
	    File file = new File("input");
        Scanner scanner = new Scanner(file);

        Pattern pattern = Pattern.compile("(se|sw|nw|ne|e|w)");

        Coordinate center = new Coordinate(0, 0);
        HashMap<Coordinate, Boolean> archive = new HashMap<>();
        archive.put(center, false);

        while (scanner.hasNextLine()) {
            Coordinate current = center;

            String line = scanner.nextLine();
            if (line.isEmpty()) continue;

            Matcher matcher = pattern.matcher(line);

            while(matcher.find()) {
                Coordinate coordinate = current.move(matcher.group());
                archive.putIfAbsent(coordinate, false);

                current = coordinate;
            }

            archive.computeIfPresent(current, (key, val) -> !val);
        }

        return archive;
    }

    public long silver() throws FileNotFoundException {
        return this.run().entrySet().stream().filter(Map.Entry::getValue).count();
    }

    public long gold() throws FileNotFoundException {
        HashMap<Coordinate, Boolean> archive = run();
        List<Coordinate> toAdd = new ArrayList<>();

        for (int i = 0; i < 100; i++) {
            HashMap<Coordinate, Boolean> flips = new HashMap<>();

            for (Map.Entry<Coordinate, Boolean> tile : archive.entrySet()) {
                toAdd.addAll(tile.getKey().getAdjacent());
            }

            for (Coordinate coordinate : toAdd) {
                archive.putIfAbsent(coordinate, false);
            }

            for (Map.Entry<Coordinate, Boolean> tile : archive.entrySet()) {
                List<Coordinate> adjacent = tile.getKey().getAdjacent();

                int black = 0;
                for (Coordinate coordinate : adjacent) {
                    if (archive.containsKey(coordinate) && archive.get(coordinate)) {
                        black++;
                    }
                }

                if (black == 0 || black > 2) {
                    flips.put(tile.getKey(), false);
                } else if (black == 2) {
                    flips.put(tile.getKey(), true);
                }
            }

            for (Map.Entry<Coordinate, Boolean> tile : flips.entrySet()) {
                archive.put(tile.getKey(), tile.getValue());
            }
        }

        return archive.entrySet().stream().filter(Map.Entry::getValue).count();
    }

    public Main() throws FileNotFoundException {
        System.out.println("Silver: " + silver());
        System.out.println("Gold: " + gold());
    }

    public static void main(String[] args) throws FileNotFoundException {
        new Main();
    }
}
