
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::env;
use std::path::Path;
use std::fs::File;


extern crate rand;
use rand::Rng;
use rand::Rand;
use std::mem::size_of;

mod instance;
mod tsp;
mod knapsack;

fn new_gen<T : Rand>() -> mpsc::Receiver<T> where T : std::marker::Send , T: 'static {
    let (send, recv) = mpsc::sync_channel(100_000 * size_of::<T>());
    let _ = thread::spawn(move || {
        let mut rng: rand::XorShiftRng = rand::random();
        while send.send(rng.gen()).is_ok() {}
    });
    recv
}

fn main() {
    let recv_rls = new_gen();
    let recv_two_opt = new_gen();
    let recv_stich = new_gen();
    let recv_knap = new_gen();
    let stdin = std::io::stdin();
    let inst = instance::read_instance(stdin.lock());
    let mut tour = tsp::new_tour_greedy(Rc::new(inst));
    println!("{:?} {:?}", &tour.cities[..10], tour.cost);
    tour.two_opt();
    tour.two_opt();
    println!("After first two-opt {}", tour.cost);
    for loop_n in 0..10 {
        println!("TSP : {:?}", loop_n);
    let mut temp = 1.0;
    let mut i = 0;
    let mut last_it = 0;
    let max_it = 1. *  4.0 * (tour.size() as f64).powi(2) * (tour.size() as f64).ln();
    while (i - last_it) <= (max_it as i64) {
        temp *= 1.0 - 1e-6;
        i += 1;
        if tour.rls_try_one(&recv_rls, temp).0 {
            println!("[ RLS ] New cost : {:?} in {} iterations ({} since last output) (temp : \
                      {:e})",
                     tour.cost,
                     i,
                     i - last_it,
                     temp);
            last_it = i;
        }
        if i % 4 == 0 {
            if tour.two_opt_rand(&recv_two_opt) {
                println!("[2-opt] New cost : {:?} in {} iterations ({} since last output) (temp \
                          : {:e})",
                         tour.cost,
                         i,
                         i - last_it,
                         temp);
                last_it = i;
            }
        }
        if i % 4 == 0 {
            if tour.stich_try_one(&recv_stich, temp).0 {
                println!("[stich] New cost : {:?} in {} iterations ({} since last output) (temp \
                          : {:e})",
                         tour.cost,
                         i,
                         i - last_it,
                         temp);
                last_it = i;
            }
        }

    }
}
    println!("Check cost : {:?}", tour.re_compute_cost());
    let mut k = knapsack::Knapsack::new(Rc::new(tour));
    for loop_n in 0..10 {
        println!("TTP: {:?}",loop_n );
    let max_it = 1. *  2.0 * (k.is_in.len() as f64) * (k.is_in.len() as f64).ln();
    let mut temp = 100.;
    let mut i = 0;
    let mut last_it = 0;
    while (i - last_it) <=  100*(max_it as i64) {
        temp *= 1.0 - 1e-5 ;
        i += 1;
        let b = k.rls_try_one(&recv_knap, temp);
        if b.1 {
            //println!("!!" );
        }
        if b.0 {
            println!("[Knap2] New val : {:?} in {} iterations ({} since last output) (temp : \
                      {:e})",
                     k.profit - k.cost,
                     i,
                     i - last_it,
                     temp);
            last_it = i;
        }
    };

//    println!("{:?}", tour.cities );
/*
    {
        let pathname = env::args().nth(1).unwrap();
        let path = Path::new(&pathname);
        let mut file = File::create(path).unwrap();
        tour.print_svg(&mut file);
    }
    */
};
}
