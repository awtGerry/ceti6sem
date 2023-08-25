import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Node {
    String name;
    List<Edge> edges;
    public Node(String name) {
        this.name = name;
        edges = new ArrayList<Edge>();
    }

    public Node recursiveSearchByDeep(String targetName, Stack<Node>toEval, List<Node> evaluated) {
        if (toEval.isEmpty()) {
            return null;
        }
        Node root = toEval.pop();
        if (root.name.equals(targetName)) {
            return root;
        }
        evaluated.add(root);
        for (Edge edge : root.edges) {
            if (!evaluated.contains(edge.target)) {
                toEval.push(edge.target);
            }
        }
        return recursiveSearchByDeep(targetName, toEval, evaluated);
    }
}
