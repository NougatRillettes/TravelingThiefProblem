
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
    let mut temp = 50.0;
    let mut i = 0;
    let mut last_it = 0;
    loop {
        temp *= 0.9999;
        i += 1;
        if tour.rls_try_one(&mut rng,temp).0 {
            println!("New cost : {:?} in {} iterations ({} since last output) (temp : {:e})",tour.cost,i,i-last_it,temp );
            last_it = i;
    }

}

}
