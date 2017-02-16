use tsp::Tour;
use instance::{Instance, euc_distance};
use std::rc::Rc;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Knapsack {
    pub rem_cap: u64,
    pub profit: f64,
    pub distances: Vec<f64>,
    pub cost: f64,
    pub tour: Rc<Tour>,
    pub is_in: Vec<bool>,
    pub city_weight: Vec<u64>, //city ordered by tour indexes
    pub city_idx_in_tour: Vec<usize>,
}

impl Knapsack {
    pub fn re_compute_cost(&self) -> f64 {
        let inst = &self.tour.instance;
        let dv = inst.min_speed - inst.max_speed;
        //println!("{:?}", self.city_weight);
        let speeds : Vec<_> = self.city_weight.iter().map(|w| ((*w as f64)/(inst.capacity as f64) * dv + inst.max_speed)).collect();
        let mut res = self.distances
            .iter()
            .zip(speeds)
            .map(|(d, v)| d / v)
            .sum();
        res *= inst.rent_ratio;
        res
    }
/*
    pub fn re_compute_profit(&self) -> f64 {
        self.is_in
            .iter()
            .zip(self.tour.instance.items.iter().map(|x| x.profit as f64))
            .filter(|&(&b, _)| b)
            .map(|(_, x)| x)
            .sum()
    }
*/
    pub fn new(t: Rc<Tour>) -> Knapsack {
        let m = t.instance.items_num;
        let n = t.size();
        let distances: Vec<f64> = {
            let coords = t.cities.iter().map(|x| t.instance.coords[*x]);
            let coords_bis = t.cities.iter().map(|x| t.instance.coords[*x]).skip(1);
            let dists = coords.zip(coords_bis);
            dists.map(|(c1, c2)| euc_distance(c1, c2)).collect()
        };
        let city_idx = {
            let mut aux = vec![0;n];
            for (i, c) in t.cities.iter().enumerate() {
                aux[*c] = i;
            }
            aux
        };
        let mut knap = Knapsack {
            rem_cap: t.instance.capacity,
            profit: 0.0,
            cost: 0.0,
            tour: t,
            is_in: vec![false; m as usize],
            city_weight: vec![0; n],
            city_idx_in_tour: city_idx,
            distances: distances,
        };
        knap.cost = knap.re_compute_cost();
        knap
    }

    pub fn rls_try_one(&mut self, c: &mpsc::Receiver<(usize, f64)>, temp: f64) -> (bool, bool) {
        let msg = c.recv().unwrap();
        let m = self.is_in.len();
        let i = msg.0 % (m - 1);
        let it_weight = self.tour.instance.items[i].weight;
        if (! self.is_in[i]) && it_weight > self.rem_cap {
            return (false,false)
        }

        let prev_cost = self.cost;
        let sign : f64 = if self.is_in[i] { -1. } else { 1. };
        let delta_profit = (sign as f64) * self.tour.instance.items[i].profit as f64;
        let idx_of_it_city_in_tour = self.city_idx_in_tour[(*self.tour.instance).items[i]
            .in_city as usize];
        for w in self.city_weight[idx_of_it_city_in_tour..].iter_mut() {
            *w = ((*w as i64) + (sign as i64) * (it_weight as i64)) as u64;
        };
        let new_cost = self.re_compute_cost();
        let delta = -new_cost + prev_cost + (delta_profit as f64);
        let improving = delta > 0.;
        //println!("delta: {:?}, i : {:?}", delta,i );
        let accepting = msg.1 <= (delta / temp).exp();
        if !accepting {
            for w in self.city_weight[idx_of_it_city_in_tour..].iter_mut() {
                *w = ((*w as i64) - (sign as i64) * (it_weight as i64)) as u64;
            };
            return (false, false);
        };
        self.profit += delta_profit;
        self.cost = new_cost;
        self.is_in[i] ^= true;
        self.rem_cap = ((self.rem_cap as i64) - (sign as i64) * (it_weight as i64)) as u64;
        (improving, accepting)
    }
}
