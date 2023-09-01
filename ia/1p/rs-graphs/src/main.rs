use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Vertice {
    id: usize,
    distance: i32,
}

impl Vertice {
    // Metodo para crear un nuevo vertice
    fn nuevo_grafo(id: usize, distance: i32) -> Self {
        Vertice { id, distance }
    }
}

fn dijkstra(grafo: &HashMap<usize, Vec<(usize, i32)>>, inicio: usize) -> HashMap<usize, i32> {
    let mut distancias: HashMap<usize, i32> = grafo.keys().map(|&k| (k, i32::MAX)).collect();
    let mut visitas: HashSet<usize> = HashSet::new();
    let mut prioridades: BinaryHeap<Vertice> = BinaryHeap::new();

    distancias.insert(inicio, 0);
    prioridades.push(Vertice::nuevo_grafo(inicio, 0));

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
                if nueva_distancia < *distancias.get(&adyacente).unwrap_or(&i32::MAX) {
                    distancias.insert(adyacente, nueva_distancia);
                    prioridades.push(Vertice::nuevo_grafo(adyacente, nueva_distancia));
                }
            }
        }
    }

    // Regresar las distancias mas cortas
    distancias
}

fn main() {
    // Create a sample graph as an adjacency list
    let mut graph: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
    graph.insert(0, vec![(1, 2), (2, 4)]);
    graph.insert(1, vec![(0, 2), (2, 1), (3, 7)]);
    graph.insert(2, vec![(0, 4), (1, 1), (3, 5)]);
    graph.insert(3, vec![(1, 7), (2, 5)]);

    let start_vertex = 0;
    let shortest_distances = dijkstra(&graph, start_vertex);

    for (vertex, distance) in shortest_distances {
        println!("Shortest distance from {} to {} is {}", start_vertex, vertex, distance);
    }
}
