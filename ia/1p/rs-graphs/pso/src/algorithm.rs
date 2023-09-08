#![allow(unused_imports)]
#![allow(unused)]

extern crate rand;

const N: usize = 40;
const X_MAX: f64 = 50.0;
const X_MIN: f64 = -50.0;

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
