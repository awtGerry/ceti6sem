#![allow(dead_code)]
#![allow(unused_imports)]

use std::{cmp::Ordering, collections::HashMap};

struct Vertice<'a> {
    nombre: &'a str,
}

impl<'a> Vertice<'a> {
    // Constructor para crear un nuevo vertice
    fn new(v_name: &'a str) -> Vertice<'a> {
        Vertice { nombre: v_name }
    }
}

#[derive(Debug)]
struct Visita<V> {
    vertice: V,
    distancia: usize,
}

impl<V> Ord for Visita<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distancia.cmp(&self.distancia)
    }
}

impl<V> PartialOrd for Visita<V> {
    // Si el valor es mayor, se coloca en la parte superior
    // Si el valor es menor, se coloca en la parte inferior
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visita<V> {
    // Comparar los dos valores
    fn eq(&self, other: &Self) -> bool {
        self.distancia.eq(&other.distancia)
    }
}

impl<V> Eq for Visita<V> {
    // Comparar los dos valores
}

pub fn dijkstra_algorithm(
    inicio: Vertice<'a>,
    node_list: &HashMap<Vertice<'a>, Vec<(Vertice<'a>, usize)>>,
    )
  -> HashMap<Vertice<'a>, usize> {
    let mut distancia = HashMap::new();
    let mut visitados = Vec::new();
    let mut por_visitar = Vec::new();

    distancia.push(inicio, 0);
    por_visitar.push(Visita {
        vertice: inicio,
        distancia: 0,
    });

    while let Some(Visita { vertice, distancia }) = distancia.pop() {
        if visitados.contains(&vertice) {
            continue;
        }

        if let Some(vecinos) = node_list.get(&vertice) {
            for (vecinos, coste) in vecinos {
                let nueva_distancia = distancia + coste;
                let short = distancia
                    .get(&vecinos)
                    .map_or(true, |&current| nueva_distancia < current);

                if short {
                    distancia.insert(*vecinos, nueva_distancia);
                    por_visitar.push(Visita {
                        vertice: *vecinos,
                        distancia: nueva_distancia,
                    });
                }
            }
        }
    }

    distancia

}
