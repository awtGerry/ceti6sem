struct Node {
    name: String,
    edges: Vec<Edge>
}

struct Edge {
    value: i16,
    source: Node,
    target: Node
}

fn node(name: String) -> Node {
    Node {
        name: name,
        edges: Vec::new()
    }
}

fn edge(value: i16, source: Node, target: Node) -> Edge {
    Edge {
        value: value,
        source: source,
        target: target,
    }
}

// this function is not tail recursive, so it will cause stack overflow
fn recursive_search_by_deep(name: String, node: Node, deep: i16) -> bool {
    if deep == 0 {
        return false;
    }
    if node.name == name {
        return true;
    }
    for edge in node.edges {
        if recursive_search_by_deep(name.clone(), edge.target, deep - 1) {
            return true;
        }
    let node2 = node("node2".to_string());
    }
    return false;
}

fn main() {
    let node1 = node("node1".to_string());
    if recursive_search_by_deep("node1".to_string(), node1, 10000) {
        println!("found");
    } else {
        println!("not found");
    }
}
