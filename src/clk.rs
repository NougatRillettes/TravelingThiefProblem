use std::slice::Iter;
use instance::*;
use std::iter::Peekable;

// Simply a Tour on cities, element of the Vec are cities indexes.
pub struct Tour<'a> {
    pub instance: &'a Instance,
    pub cost: u64,
    pub cities: Vec<usize>,
}

pub fn new_tour(inst: &Instance) -> Tour {
    let mut tour = Tour {
        instance: inst,
        cost: 0,
        cities: Vec::new(),
    };
    let prev_c = inst.coords[0];
    for (i, &c) in inst.coords.iter().enumerate() {
        tour.cities.push(i);
        tour.cost += sqr_distance(c, prev_c);
    }
    tour
}

pub fn new_tour_greedy(inst: &Instance) -> Tour {
    let mut tour = Tour {
        instance: inst,
        cost: 0,
        cities: Vec::new(),
    };
    let n = tour.instance.coords.len();
    let mut curr_city = inst.coords[0];
    let mut done = vec![false;n];
    done[0] = true;
    tour.cities.push(0);
    while tour.cities.len() < n {
        {
            let coord_iter = tour.instance.coords.iter().enumerate();
            let dist_iter = coord_iter.map(|(i, &x)| (i, sqr_distance(curr_city, x)));
            let done_iter = done.iter_mut();
            let zip_iter = dist_iter.zip(done_iter);
            let rem_dist = zip_iter.filter(|&(_, &mut y)| !y);
            let nearest = rem_dist.min_by_key(|&((_, x), _)| x).unwrap();
            *nearest.1 = true;
            tour.cities.push((nearest.0).0);
            tour.cost += (nearest.0).1;
        }
        curr_city = inst.coords[tour.cities[tour.cities.len() - 1]];
    }
    //
    tour
}



// An Iterator on the crossing edges of a Tour (ie those whose
// cost can be reduced by 2-opt
pub struct TourCrossing<'a> {
    tour: &'a Tour<'a>,
    left: usize,
    right: usize,
}

impl<'a> Iterator for TourCrossing<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.tour.size();

            if self.left >= n {
                return None;
            }
            if self.right >= n - 1 {
                self.left += 1;
                self.right = self.left + 1;
                continue;
            }
            let instance = self.tour.instance;
            let city1a = instance.coords[self.tour.cities[self.left - 1]];
            let city1b = instance.coords[self.tour.cities[self.left]];
            let city2a = instance.coords[self.tour.cities[self.right]];
            let city2b = instance.coords[self.tour.cities[self.right + 1]];
            if sqr_distance(city1a, city1b) + sqr_distance(city2a, city2b) >
               sqr_distance(city1a, city2a) + sqr_distance(city1b, city2b) {
                self.right += 1;
                return Some((self.left, self.right - 1));
            }
            self.right += 1;
        }
    }
}

impl<'b> Tour<'b> {
    pub fn size(&self) -> usize {
        self.cities.len()
    }
    pub fn crossings<'a>(&'a self) -> TourCrossing<'a> {
        TourCrossing {
            tour: self,
            left: 1,
            right: 2,
        }
    }
}
