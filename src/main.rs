
use std::io::prelude::*;

mod instance;
mod tsp;

fn main() {
    let stdin = std::io::stdin();
    let inst = instance::read_instance(stdin.lock());
    let tour = tsp::new_tour_greedy(&inst);
    println!("{:?} {:?}", &tour.cities[..10], tour.cost);
    let crossings = tour.crossings();
    for x in crossings {
        println!("{:?}", x);
    }


}
