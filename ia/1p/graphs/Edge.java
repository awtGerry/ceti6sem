public class Edge {
    int value;
    Node source;
    Node target;
    public Edge(Node source, Node destination, int value) {
        this.source = source;
        this.target = destination;
        this.value = value;
    }
}
