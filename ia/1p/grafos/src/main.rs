struct Node {
    name: String,
    edges: Vec<Edge>
}

struct Edge {
    value: i16,
    source: Node,
    target: Node
}

fn Node(name: String) -> Vec<Edge> {
    Node {
        name: name,
        edges: Vec::new()
    }
}

fn Edge(value: i16, source: Node, target: Node) -> Edge {
    Edge {
        value: value,
        source: source,
        target: target
    }
}

fn recursive_search_by_deep(targetName: String, node_to_eval: Node, deep: i16) -> bool {
    if deep == 0 {
        return false;
    }
    if node_to_eval.name == targetName {
        return true;
    }
    for edge in node_to_eval.edges {
        if recursive_search_by_deep(targetName, edge.target, deep - 1) {
            return true;
        }
    }
    return false;
}
}

fn main() {
    let a = Node("A".to_string());
    let b = Node("B".to_string());
    let ab = Edge(2, a, b);
    let c = Node("C".to_string());
    let ca = Edge(3, c, a);
    a.edges.push(ab);
    c.edges.push(ca);
    if (recursive_search_by_deep("B".to_string(), c, 2)) {
        println!("Found!");
    } else {
        println!("Not found!");
    }
}
