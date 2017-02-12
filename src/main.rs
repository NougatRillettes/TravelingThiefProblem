
use std::io::prelude::*;
use std::rc::Rc;

extern crate rand;

mod instance;
mod tsp;

fn main() {
    let mut rng : rand::XorShiftRng = rand::random();
    let stdin = std::io::stdin();
    let inst = instance::read_instance(stdin.lock());
    let mut tour = tsp::new_tour_greedy(Rc::new(inst));
    println!("{:?} {:?}", &tour.cities[..10], tour.cost);
    loop {
    let mut i = 0;
    while !tour.rls_try_one(&mut rng) {
        i += 1;
    }
    println!("New cost : {:?} in {} iterations",tour.cost,i );
}

}
