
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::env;
use std::path::Path;
use std::fs::File;


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
    tour.two_opt();
    tour.two_opt();
    println!("After first two-opt {}", tour.cost);
    let mut temp = 1.0;
    let mut i = 0;
    let mut last_it = 0;
    let max_it = 4.0 * (tour.size() as f64).powi(2) * (tour.size() as f64).ln();
    while (i - last_it) <= (max_it as i64) {
        temp *= 1.0 - 1e-6;
        i += 1;
        if tour.rls_try_one(&recv, temp).0 {
            println!("[ RLS ] New cost : {:?} in {} iterations ({} since last output) (temp : \
                      {:e})",
                     tour.cost,
                     i,
                     i - last_it,
                     temp);
            last_it = i;
        }
        if i % 2 == 0 {
            if tour.two_opt_rand(&recv) {
                println!("[2-opt] New cost : {:?} in {} iterations ({} since last output) (temp \
                          : {:e})",
                         tour.cost,
                         i,
                         i - last_it,
                         temp);
                last_it = i;
            }
        }

    }
    tour.two_opt();
    println!("After two-opt {}", tour.cost);

    println!("Check cost : {:?}", tour.re_compute_cost());
    {
        let pathname = env::args().nth(1).unwrap();
        let path = Path::new(&pathname);
        let mut file = File::create(path).unwrap();
        tour.print_svg(&mut file);
    }
}
