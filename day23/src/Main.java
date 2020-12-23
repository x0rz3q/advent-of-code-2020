import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Main {
    // Just a simple record like structure to keep track of everything
    public static class Node {
        public Node next = null;
        public Node previous = null;
        public int value;
    }

    private int[] input;

    public Main(int[] input) {
        this.input = input;
    }

    public Node findLocation(List<Node> nodes, Node current, Node first, Node second, Node third) {
        int index = current.value - 1;

        while (index == first.value || index == second.value || index == third.value) {
            index--;
        }

        if (index >= 1) {
            return nodes.get(index - 1);
        }

        index = nodes.size();

        while (index == first.value || index == second.value || index == third.value) {
            index--;
        }

        return nodes.get(index - 1);
    }

    public Node run(List<Node> nodes, int steps) {
        Node current = nodes.get(0);
        List<Node> sorted = new ArrayList<>(nodes);

        sorted.sort(Comparator.comparingInt(a -> a.value));

        for (int i = 0; i < steps; i++) {
            Node first = current.next;
            Node second = first.next;
            Node third = second.next;

            // Disconnect the cups from the main list
            current.next = third.next;
            third.next.previous = current;

            Node destination = this.findLocation(sorted, current, first, second, third);

            // Link everything back together
            third.next = destination.next;
            destination.previous = third;
            destination.next = first;
            first.previous = destination;

            current = current.next;
        }

        return sorted.get(0);
    }

    public String silver() {
        Node current = this.run(this.getNodes(), 100);

        StringBuilder builder = new StringBuilder();
        for (int i = 1; i < this.input.length; i++) {
            current = current.next;

            builder.append(current.value);
        }

        return builder.toString();
    }

    public long gold() {
        // Make the big list of nodes
        List<Node> nodes = new ArrayList<>(this.getNodes());
        Node last = nodes.get(nodes.size() - 1);

        for (int i = 10; i <= 1000000; i++) {
            Node node = new Node();

            node.value = i;
            node.previous = last;
            last.next = node;

            last = node;

            nodes.add(node);
        }

        last.next = nodes.get(0);
        nodes.get(0).previous = last;

        Node current = nodes.get(0);
        current.previous = last;
        last.next = current;

        current = this.run(nodes, 10000000);

        return (long)current.next.value * (long)current.next.next.value;
    }

    public List<Node> getNodes() {
        List<Node> nodes = new ArrayList<>();

        for (int i = 0; i < this.input.length; i++) {
            Node node = new Node();
            node.value = this.input[i];

            if (nodes.size() != 0) {
                Node previous = nodes.get(i - 1);
                previous.next = node;
                node.previous = previous;
            }

            nodes.add(node);
        }

        nodes.get(0).previous = nodes.get(nodes.size() - 1);
        nodes.get(nodes.size() - 1).next = nodes.get(0);

        return nodes;
    }

    public void run() {
        System.out.println("Silver: " + silver());
        System.out.println("Gold: " + gold());
    }

    public static void main(String[] args) {
        int[] input = {5, 3, 8, 9, 1, 4, 7, 6, 2};

        (new Main(input)).run();
    }
}
