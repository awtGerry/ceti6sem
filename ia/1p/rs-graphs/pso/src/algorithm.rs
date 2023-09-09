#![allow(unused_imports)]
#![allow(unused)]

extern crate rand;

use rand::Rng;

const N: usize = 10;
const ITERACIONES: u16 = 100;
const X_MAX: f64 = 5.12;
const X_MIN: f64 = -5.12;
const C1: f64 = 2.0;
const C2: f64 = 0.2;
const W: f64 = 0.9;
const A: f64 = 0.9;
const T: f64 = 1.0;

type Abejas = Vec<Abeja>;

#[derive(Debug)]
struct Abeja {
    posicion: (f64, f64),
    velocidad: (f64, f64),
    mejor_pos: (f64, f64),
}

pub fn pso() {
    let mut abejas = iniciar(N, X_MAX, X_MIN, A, T);
    let mut mejor_abeja_pos = (X_MIN, X_MAX);

    for i in 0..ITERACIONES {
        let abejas_pos: Vec<f64> = abejas.iter().map(|x| evaluar(x.posicion)).collect();
        let abejas_mejor_pos: Vec<f64> = abejas.iter().map(|x| evaluar(x.mejor_pos)).collect();

        actualizar(&mut abejas, &mut mejor_abeja_pos, &abejas_pos, &abejas_mejor_pos, i as usize);

        if 1.0 / evaluar(mejor_abeja_pos) < 0.0001 {
            println!("Salio en iteracion {:?}", i);
            break;
        }
    }    
    let mejor_abeja = evaluar(mejor_abeja_pos);
    println!("Mejor abeja: {:?} con valor {:?}", mejor_abeja_pos, mejor_abeja);
}

fn iniciar(
    n: usize,
    x_max: f64,
    x_min: f64,
    a: f64,
    t: f64,
) -> Abejas {
    let mut particulas: Vec<Abeja> = Vec::with_capacity(n);
    let mut random = rand::thread_rng();

    for _ in 0..n {
        let r1: f64 = random.gen();
        let r2: f64 = random.gen();
        let r3: f64 = random.gen();
        let r4: f64 = random.gen();

        let x1 = x_min + r1 * (x_max - x_min);
        let x2 = x_min + r2 * (x_max - x_min);
        let v1 = a/t*(-(x_max-x_min)/2.0 + r3*(x_max-x_min));
        let v2 = a/t*(-(x_max-x_min)/2.0 + r4*(x_max-x_min));

        let particula = Abeja {
            posicion: (x1, x2),
            velocidad: (v1, v2),
            mejor_pos: (x1, x2),
        };

        particulas.push(particula);
    }

    println!("{:?}", particulas);
    particulas
}

fn evaluar((x, y): (f64, f64)) -> f64 {
    1.0 / (x*x + y - 11.0).powi(2) + (x + y*y - 7.0).powi(2)
}

fn actualizar(
    particulas: &mut Abejas,
    mejor_abeja_pos: &mut (f64, f64),
    x_particula: &Vec<f64>,
    x_mejor_particula: & Vec<f64>,
    iter: usize,
) {
    let mut mejor_abeja = evaluar(*mejor_abeja_pos);

    for i in 0..particulas.len() {
        if x_particula[i] > x_mejor_particula[i] {
            particulas[i].mejor_pos = particulas[i].posicion;

            if x_particula[i] > mejor_abeja {
                *mejor_abeja_pos = particulas[i].posicion;
                mejor_abeja = evaluar(*mejor_abeja_pos);
                println!("Mejor abeja: {:?} en iteracion {:?}", mejor_abeja_pos, iter);
            }
        }
    }
}
