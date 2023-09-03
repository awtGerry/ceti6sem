use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Vertice<'a> {
    id: &'a str,
    distance: i32,
}

impl<'a> Vertice<'a>{
    // Metodo para crear un nuevo vertice
    fn new_graph(id: &'a str, distance: i32) -> Self {
        Vertice { id, distance }
    }
}

fn dijkstra<'a>(
    nodo: &HashMap<&'a str, Vec<(&'a str, i32)>>,
    inicio: &'a str,
    fin: &'a str,
    ) -> HashMap<&'a str, i32>
{
    let mut distancias: HashMap<&str, i32> = nodo.keys().map(|&x| (x, i32::MAX)).collect();
    let mut visitas: HashSet<&str> = HashSet::new();
    let mut prioridades: BinaryHeap<Vertice> = BinaryHeap::new();
    let mut shortest_path: HashMap<&str, &str> = HashMap::new();

    distancias.insert(inicio, 0);
    prioridades.push(Vertice::new_graph(inicio, 0));

    // Encontrar el camino mas corto desde el inicio hasta el fin y regresar las distancias
    while !prioridades.is_empty() {
        let Vertice { id, distance } = prioridades.pop().unwrap();
        if id == fin {
            break;
        }
        if visitas.contains(id) {
            continue;
        }
        visitas.insert(id);
        for (vecino, peso) in &nodo[id] {
            let peso = distance + peso;
            if peso < distancias[vecino] {
                distancias.insert(vecino, peso);
                shortest_path.insert(vecino, id);
                prioridades.push(Vertice::new_graph(vecino, peso));
            }
        }
    }

    // Regresar el camino mas corto desde el inicio hasta el fin
    let mut path = vec![fin];
    let mut current = fin;
    while current != inicio {
        current = shortest_path[current];
        path.push(current);
    }
    println!("El camino mas corto desde {} hasta {} es: {:?}", inicio, fin, path);

    distancias
}

fn main() {
    // Ejemplo
    let mut graph: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();
    graph.insert("A", vec![("B", 3), ("D", 8)]);
    graph.insert("B", vec![("D", 5), ("E", 6)]);
    graph.insert("D", vec![("B", 5), ("E", 3), ("F", 2)]);
    graph.insert("E", vec![("F", 1), ("C", 9)]);
    graph.insert("F", vec![("E", 1), ("C", 3)]);
    graph.insert("C", vec![("E", 9), ("F", 3)]);

    let inicio = "A";
    let fin = "C";
    dijkstra(&graph, inicio, fin);

    for (vertice, distance) in result {
        println!("Distancia mas corta de {} hacia {} es: {}", inicio, vertice, distance);
    }
}
