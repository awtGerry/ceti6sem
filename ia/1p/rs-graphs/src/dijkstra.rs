#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;

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
struct Visit<V> {
    vertice: V,
    distancia: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distancia.cmp(&self.distancia)
    }
}

impl<V> PartialOrd for Visit<V> {
    // Si el valor es mayor, se coloca en la parte superior
    // Si el valor es menor, se coloca en la parte inferior
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    // Comparar los dos valores
    fn eq(&self, other: &Self) -> bool {
        self.distancia.eq(&other.distancia)
    }
}

impl<V> Eq for Visit<V> {
    // Comparar los dos valores
}

pub fn dijkstra(
    inicio: Vertice<'a>,
    lista: Vec<Vertice<'a>, Vec<(Vertice<'a>, usize)>>,
) -> Vec<Vertice<'a>, usize>
{
    let distancia = Vec::new();
    let visitado;
    let to_visitar;
}
