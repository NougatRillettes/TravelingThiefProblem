
use std::io::prelude::*;
use std::rc::Rc;

mod instance;
mod tsp;

fn main() {
    let stdin = std::io::stdin();
    let inst = instance::read_instance(stdin.lock());
    let tour = tsp::new_tour_greedy(Rc::new(inst));
    println!("{:?} {:?}", &tour.cities[..10], tour.cost);
    let crossings = tour.crossings();
    for x in crossings {
        println!("{:?}", x);
    }


}
