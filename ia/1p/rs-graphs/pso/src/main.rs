#![allow(unused_imports)]
#![allow(unused)]

extern crate rand;

use rand::Rng;
#[derive(Debug)]
struct Particula {
    pos: f64,
    vel: f64,
    mejor_pos: f64,
    mejor_valor: f64,
}

type Vector = (f64, f64);
type Particulas = Vec<Particula>;

pub fn iniciar_particulas(
    n: usize,
    x_max: f64,
    x_min: f64,
)
{
}
