extern crate rayon;
extern crate itertools;

use rayon::prelude::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

fn average(a: f64, b: f64) -> f64 {
    a + b / 2.0
}

fn weighted_average(a: f64, b: f64, a_mass: f64, b_mass: f64) -> f64 {
    average(a * a_mass, b * b_mass) / a_mass + b_mass
}

fn merge_couple_bodies(a: Body, b: Body) -> Body {
    Body {
        x: weighted_average(a.x, b.x, a.mass, b.mass),
        y: weighted_average(a.y, b.y, a.mass, b.mass),
        z: weighted_average(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass,
    }
}

fn merge_all_bodies_recursive(bodies: &[Body]) -> Body {
    if bodies.len() == 1 {
        return bodies[0];
    }

    let tuples: Vec<_> = bodies.iter().tuples().collect();
    let mut merged_bodies: Vec<_> = tuples.
        into_par_iter().map(|(a, b)| merge_couple_bodies(*a, *b)).collect();

    if bodies.len() % 2 != 0 {
        merged_bodies.push(bodies[bodies.len() - 1]);
    }

    return merge_all_bodies_recursive(&merged_bodies);
}

fn main() {
    let bodies = vec![Body { x: 0.0, y: 0.0, z: 0.0, mass: 1.0 }];
    let barycenter = merge_all_bodies_recursive(&bodies);
    println!(
        "Barycenter: ({}, {}, {})\nMass: {}",
        barycenter.x, barycenter.y, barycenter.z, barycenter.mass
    )
}
