import java.util.*;

public class Graph {
    List<Node> nodes;
    Node start;

    public Graph() {
        this.nodes = new ArrayList<Node>();
        Node a = new Node("A");
        Node b = new Node("B");
        Edge ab = new Edge(a, b, 2);
        nodes.add(a);
        nodes.add(b);
        this.start = a;
        Node c = new Node("C");
        Edge ca = new Edge(c, a, 3);
        nodes.add(c);
        a.edges.add(ab);
        c.edges.add(ca);
    }
}
