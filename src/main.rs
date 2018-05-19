extern crate rand;
use rand::{thread_rng, Rng};
use std::f64;
use std::num;

fn pi(total: i64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut count_inside = 0f64;

    for _ in 0..total {
        let x1: f64 = rng.gen();
        let x2: f64 = rng.gen();
        let dist = (x1.powi(2) + x2.powi(2)).sqrt();
        if dist < 1.0 {
            count_inside += 1.0;
        }
    }
    4.0 * count_inside / total as f64
}

fn main() {
    // Rng is the main trait and needs to be imported:
    println!("{}", pi(100000000));
}
