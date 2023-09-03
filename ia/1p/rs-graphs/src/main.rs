use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Vertice<'a> {
    id: &'a str,
    distance: i32,
}

impl Vertice<'_>{
    // Metodo para crear un nuevo vertice
    fn new_graph(id: &str, distance: i32) -> Self {
        Vertice { id, distance }
    }
}

fn dijkstra(grafo: &HashMap<&str, Vec<(usize, i32)>>, inicio: &str) -> HashMap<&str, i32> {
    let mut distancias: HashMap<&str, i32> = grafo.keys().map(|&k| (k, i32::MAX)).collect();
    let mut visitas: HashSet<&str> = HashSet::new();
    let mut prioridades: BinaryHeap<Vertice> = BinaryHeap::new();

    distancias.insert(inicio, 0);
    prioridades.push(Vertice::new_graph(inicio, 0));

    // Mientras haya vertices por visitar en el heap
    while let Some(vertice_actual) = prioridades.pop() {
        // Validar que el vertice no haya sido visitado
        if visitas.contains(&vertice_actual.id) {
            continue;
        }

        // Marcar el vertice actual como visitado
        visitas.insert(vertice_actual.id);

        // Actualizar las distancias de los vertices adyacentes
        if let Some(adyacentes) = grafo.get(&vertice_actual.id) {
            for &(adyacente, peso) in adyacentes {
                let nueva_distancia = vertice_actual.distance + peso;
                if nueva_distancia < *distancias.get(&adyacente).unwrap() {
                    distancias.insert(adyacente, nueva_distancia);
                    prioridades.push(Vertice::new_graph(adyacente, nueva_distancia));
                }
            }
        }
    }

    // Regresar las distancias mas cortas
    distancias
}

fn main() {
    // Grafo de ejemplo
    let mut graph: HashMap<&str, Vec<(usize, i32)>> = HashMap::new();
    // Se insertan los vertices y sus aristas con el peso de cada una
    graph.insert("A", vec![(1, 2), (2, 4)]);
    graph.insert("B", vec![(0, 2), (2, 1), (3, 7)]);
    graph.insert("C", vec![(0, 4), (1, 1), (3, 5)]);
    graph.insert("D", vec![(1, 7), (2, 5)]);
    graph.insert("E", vec![(1, 7), (2, 5)]);

    let vertice_inicio = "E";
    let shortest_distances = dijkstra(&graph, vertice_inicio);

    for (vertex, distance) in shortest_distances {
        println!("Shortest distance from {} to {} is {}", vertice_inicio, vertex, distance);
    }
}
