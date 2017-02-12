
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

extern crate rand;
use rand::Rng;

mod instance;
mod tsp;

fn main() {
    let (send, recv) = mpsc::sync_channel(100_000 * 3 * 64 / 8);
    let _ = thread::spawn(move || {
        let mut rng: rand::XorShiftRng = rand::random();
        while send.send(rng.gen()).is_ok() {}
    });
    let stdin = std::io::stdin();
    let inst = instance::read_instance(stdin.lock());
    let mut tour = tsp::new_tour_greedy(Rc::new(inst));
    println!("{:?} {:?}", &tour.cities[..10], tour.cost);
    let mut temp = 1.0;
    let mut i = 0;
    let mut last_it = 0;
    let max_it = 4.0 * (tour.size() as f64).powi(2) * (tour.size() as f64).ln();
    while (i - last_it) <= (max_it as i64) {
        temp *= 0.95;
        i += 1;
        if tour.rls_try_one(&recv, temp).0 {
            println!("New cost : {:?} in {} iterations ({} since last output) (temp : {:e})",
                     tour.cost,
                     i,
                     i - last_it,
                     temp);
            last_it = i;
        }
    }

}
