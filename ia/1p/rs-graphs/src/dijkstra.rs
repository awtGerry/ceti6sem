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

pub fn dijkstra<'a>(
    nodo: &HashMap<&'a str, Vec<(&'a str, i32)>>,
    inicio: &'a str,
    fin: &'a str,
    ) -> Vec<&'a str>
{
    let mut distancias: HashMap<&str, i32> = nodo.keys().map(|&x| (x, i32::max_value())).collect();
    let mut visitas: HashSet<&str> = HashSet::new();
    let mut prioridades: BinaryHeap<Vertice> = BinaryHeap::new();
    let mut shortest_path: HashMap<&str, &str> = HashMap::new();

    let mut path = vec![fin];
    let mut current = fin;

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

    while current != inicio {
        current = shortest_path[current];
        path.push(current);
    }

    path
}
