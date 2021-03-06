use std::slice::Iter;
use instance::*;
use std::iter::Peekable;
use std::rc::Rc;
use std::sync::mpsc;
use std::io::Write;

extern crate rand;

// Simply a Tour on cities, element of the Vec are cities indexes.
#[derive(Debug)]
pub struct Tour {
    pub instance: Rc<Instance>,
    pub cost: f64,
    // Circuits are encoded as an n-vector.
    // /!\ There is n+1 edges : count the wrapping one.
    pub cities: Vec<usize>,
}

pub fn new_tour(inst: Rc<Instance>) -> Tour {
    let mut tour = Tour {
        instance: inst.clone(),
        cost: 0.0,
        cities: Vec::new(),
    };
    let prev_c = inst.coords[0];
    for (i, &c) in inst.coords.iter().enumerate() {
        tour.cities.push(i);
        tour.cost += euc_distance(c, prev_c);
    }
    tour.cost += euc_distance(inst.coords[tour.cities[tour.size() - 1]],
                              inst.coords[tour.cities[0]]);
    tour
}

pub fn new_tour_greedy(inst: Rc<Instance>) -> Tour {
    let mut tour = Tour {
        instance: inst.clone(),
        cost: 0.0,
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
            let dist_iter = coord_iter.map(|(i, &x)| (i, sq_distance(curr_city, x)));
            let done_iter = done.iter_mut();
            let zip_iter = dist_iter.zip(done_iter);
            let rem_dist = zip_iter.filter(|&(_, &mut y)| !y);
            let nearest = rem_dist.min_by_key(|&((_, x), _)| x).unwrap();
            *nearest.1 = true;
            tour.cities.push((nearest.0).0);
            tour.cost += ((nearest.0).1 as f64).sqrt();
        }
        curr_city = inst.coords[tour.cities[tour.cities.len() - 1]];
    }
    //
    tour.cost += euc_distance(inst.coords[tour.cities[tour.size() - 1]],
                              inst.coords[tour.cities[0]]);
    tour
}

impl Tour {
    pub fn size(&self) -> usize {
        self.cities.len()
    }

    pub fn rls_try_one(&mut self,
                       c: &mpsc::Receiver<(usize, usize, f64)>,
                       temp: f64)
                       -> (bool, bool) {
        let n = self.size();
        let msg = c.recv().unwrap();
        let mut i = (msg.0 % (n - 1)) + 1; // in [1,n-1]
        let mut j = (msg.1 % (n - 2)) + 1;
        if j >= i {
            j += 1;
        }
        {
            use std::cmp::{min, max};
            let tmp = min(i, j);
            j = max(i, j);
            i = tmp;
        }
        let delta: f64 = {
            let dist_bewteen = |a: usize, b: usize| {
                let coord_a = self.instance.coords[self.cities[a]];
                let coord_b = self.instance.coords[self.cities[b]];
                let tmp = euc_distance(coord_a, coord_b);
                tmp as f64
            };
            let di_1 = dist_bewteen(i - 1, j) - dist_bewteen(i - 1, i);
            let di_2 = dist_bewteen(j, i + 1) - dist_bewteen(i, i + 1);
            let dj_1 = dist_bewteen(j - 1, i) - dist_bewteen(j - 1, j);
            let dj_2 = dist_bewteen(i, (j + 1) % n) - dist_bewteen(j, (j + 1) % n);
            di_1 + dj_2 +
            {
                if (i + 1) == j { 0. } else { dj_1 + di_2 }
            }
        };
        let improving = delta < 0.;
        let accepting = msg.2 <= (-delta / temp).exp();
        if accepting {
            self.cities.swap(i, j);
            self.cost += delta as f64;
        }
        (improving, accepting)
    }

    pub fn stich_try_one(&mut self,
                         c: &mpsc::Receiver<(usize, usize, f64, usize, bool)>,
                         temp: f64)
                         -> (bool, bool) {
        let n = self.size();
        let msg = c.recv().unwrap();
        let x = (msg.0 % (n - 2)) + 1; // in [1,n-2]
        let y = (msg.1 % (n - 2)) + 1;
        let z = (msg.3 % (n - 2)) + 1;
        let i;
        let j;
        let a;
        let mut v = vec![x, y, z];
        v.sort();
        if msg.4 {
            a = v[0];
            i = v[1];
            j = v[2];
        } else {
            i = v[0];
            j = v[1];
            a = v[2];
        }
        if i - 1 <= a && a <= j + 1 {
            return (false, false);
        }
        let delta: f64 = {
            let dist_bewteen = |a: usize, b: usize| {
                let coord_a = self.instance.coords[self.cities[a]];
                let coord_b = self.instance.coords[self.cities[b]];
                let tmp = euc_distance(coord_a, coord_b);
                tmp as f64
            };
            dist_bewteen(a, i) + dist_bewteen(j, a + 1) + dist_bewteen(i - 1, j + 1) -
            dist_bewteen(a, a + 1) - dist_bewteen(i - 1, i) - dist_bewteen(j, j + 1)
        };
        let improving = delta < -0.0;
        let accepting = msg.2 <= (-delta / temp).exp();
        if accepting {
            //println!("{:?}", (a,i,j,delta));
            let mut new_cities: Vec<usize> = Vec::new();
            if a < i {
                new_cities.extend(&self.cities[..a + 1]);
                new_cities.extend(&self.cities[i..j + 1]);
                new_cities.extend(&self.cities[a + 1..i]);
                new_cities.extend(&self.cities[j + 1..]);
            } else {
                new_cities.extend(&self.cities[..i]);
                new_cities.extend(&self.cities[j + 1..a + 1]);
                new_cities.extend(&self.cities[i..j + 1]);
                new_cities.extend(&self.cities[a + 1..]);
            }
            self.cities = new_cities;
            self.cost += delta;

        }
        (improving, accepting)
    }

    pub fn two_opt_rand(&mut self, c: &mpsc::Receiver<(usize, usize)>) -> bool {
        let n = self.size();
        let msg = c.recv().unwrap();
        let mut i = (msg.0 % (n - 1)) + 1; // in [1,n-1]
        let mut j = (msg.1 % (n - 2)) + 1;
        if j >= i {
            j += 1;
        }
        {
            use std::cmp::{min, max};
            let tmp = min(i, j);
            j = max(i, j);
            i = tmp;
        }
        self.two_opt_at(i, j)
    }

    pub fn two_opt_at(&mut self, left: usize, right: usize) -> bool {
        let mut changed = false;
        let n = self.size();
        let instance = &self.instance;
        let city1a = instance.coords[self.cities[left - 1]];
        let city1b = instance.coords[self.cities[left]];
        let city2a = instance.coords[self.cities[right]];
        let city2b = instance.coords[self.cities[(right + 1) % n]];
        let delta = euc_distance(city1a, city1b) + euc_distance(city2a, city2b) -
                    euc_distance(city1a, city2a) - euc_distance(city1b, city2b);
        if delta > 0.01 {
            let mut i = left;
            let mut j = right;
            while i < j {
                self.cities.swap(i, j);
                i += 1;
                j -= 1;
            }
            self.cost -= delta;
            changed = true;
        }
        changed
    }

    pub fn two_opt(&mut self) -> bool {
        let mut left = 1;
        let mut right = 2;
        let mut changed = false;
        let n = self.size();
        while left < n {
            if right >= n {
                left += 1;
                right = left + 1;
                continue;
            }
            changed |= self.two_opt_at(left, right);
            right += 1;
        }
        changed
    }

    pub fn re_compute_cost(&self) -> f64 {
        let mut res = 0.;
        let mut last_city = 0;
        for c in self.cities.iter() {
            res += euc_distance(self.instance.coords[*c], self.instance.coords[last_city]);
            last_city = *c;
        }
        res += euc_distance(self.instance.coords[last_city], self.instance.coords[0]);
        res
    }

    pub fn print_svg<B: Write>(&self, b: &mut B) {
        writeln!(b,
                 "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<svg \
                  xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">")
            .unwrap();
        let mut last_city = 0;
        for c in &self.cities[1..] {
            let (x1, y1) = self.instance.coords[last_city];
            let (x2, y2) = self.instance.coords[*c];
            writeln!(b,
                     "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" \
                      stroke-width=\"2\" />",
                     x1,
                     y1,
                     x2,
                     y2)
                .unwrap();
            writeln!(b,
                     "<circle cx=\"{}\" cy=\"{}\" r=\"2\" fill=\"{}\" />",
                     x1,
                     y1,
                     "blue")
                .unwrap();
            last_city = *c;
        }
        let (x1, y1) = self.instance.coords[last_city];
        let (x2, y2) = self.instance.coords[0];
        writeln!(b,
                 "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" \
                  stroke-width=\"4\" />",
                 x1,
                 y1,
                 x2,
                 y2)
            .unwrap();
        writeln!(b,
                 "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" />",
                 x2,
                 y2,
                 "red")
            .unwrap();
        writeln!(b, "</svg>").unwrap();
    }
}

/* LEGACY


// An Iterator on the crossing edges of a Tour (ie those whose
// cost can be reduced by 2-opt
pub struct TourCrossing<'a> {
    tour: &'a Tour,
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
            let instance = &self.tour.instance;
            let city1a = instance.coords[self.tour.cities[self.left - 1]];
            let city1b = instance.coords[self.tour.cities[self.left]];
            let city2a = instance.coords[self.tour.cities[self.right]];
            let city2b = instance.coords[self.tour.cities[self.right + 1]];
            if euc_distance(city1a, city1b) + euc_distance(city2a, city2b) >
               euc_distance(city1a, city2a) + euc_distance(city1b, city2b) {
                self.right += 1;
                return Some((self.left, self.right - 1));
            }
            self.right += 1;
        }
    }
}

impl Tour {
    pub fn crossings<'a>(&'a self) -> TourCrossing<'a> {
        TourCrossing {
            tour: self,
            left: 1,
            right: 2,
        }
    }
}
*/
