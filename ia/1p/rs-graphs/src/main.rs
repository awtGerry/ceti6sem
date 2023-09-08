use std::collections::HashMap;

mod dijkstra;
mod pso;

#[allow(unused)]
fn call_dijkstra() {
    let mut graph: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();
    graph.insert("A", vec![("B", 3), ("D", 8)]);
    graph.insert("B", vec![("D", 5), ("E", 6)]);
    graph.insert("D", vec![("B", 5), ("E", 3), ("F", 2)]);
    graph.insert("E", vec![("F", 1), ("C", 9)]);
    graph.insert("F", vec![("E", 1), ("C", 3)]);
    graph.insert("C", vec![("E", 9), ("F", 3)]);

    let inicio = "A";
    let fin = "C";
    let res = dijkstra::dijkstra(&graph, inicio, fin);
    println!("El camino mas corto desde {} hasta {} es: {:?}", inicio, fin, res);
}

fn call_pso() {
    let n = 40 as usize;
    const X_MAX: f64 = 5.0;
    const X_MIN: f64 = -5.0;
}

fn main() {
    call_pso();
}
