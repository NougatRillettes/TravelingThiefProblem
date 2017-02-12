use std::vec;
use std::str::FromStr;
use std::fmt::Debug;
use std::io::prelude::*;
use std::iter;
use std::io;

#[derive(Debug)]
pub struct Item {
    pub index: u64,
    pub profit: i64,
    pub weight: u64,
    pub in_city: u64,
}

type Coord = (i64, i64);

pub fn sq_distance(c1: Coord, c2: Coord) -> u64 {
    let (x, y) = c1;
    let (a, b) = c2;
    ((a - x)*(a - x) + (b - y)*(b - y)) as u64
}

pub fn euc_distance(c1 : Coord, c2: Coord) -> f64 {
    (sq_distance(c1,c2) as f64).sqrt()
}

#[derive(Debug, Default)]
pub struct Instance {
    pub name: String,
    pub dimension: u64,
    pub capacity: u64,
    pub items_num: u64,
    pub min_speed: f64,
    pub max_speed: f64,
    pub rent_ratio: f64,
    pub coords: Vec<Coord>,
    pub items: Vec<Item>,
}

fn read_val_at<F: FromStr, I: Iterator<Item = String>>(s: &mut I, n: usize) -> F
    where F::Err: Debug
{
    let next_str = s.next().unwrap();
    let mut words = next_str.split_whitespace();
    let to_parse = words.nth(n).unwrap();
    to_parse.parse::<F>().unwrap()
}

fn read_coords(s: &str) -> Coord {
    let mut words = s.split_whitespace();
    let x = words.nth(1).unwrap().parse().unwrap();
    let y = words.next().unwrap().parse().unwrap();
    (x, y)
}

fn read_item(s: &str) -> Item {
    let mut words = s.split_whitespace();
    let idx = words.nth(0).unwrap().parse().unwrap();
    let profit = words.nth(0).unwrap().parse().unwrap();
    let weight = words.nth(0).unwrap().parse().unwrap();
    let city = words.nth(0).unwrap().parse().unwrap();
    Item {
        index: idx,
        profit: profit,
        weight: weight,
        in_city: city,
    }
}

pub fn read_instance<R: BufRead>(f: R) -> Instance {
    let mut lines = f.lines().map(|x| x.unwrap());
    let mut instance: Instance = Default::default();
    instance.name = lines.next().unwrap().split_whitespace().nth(2).unwrap().to_string();
    lines.next();
    instance.dimension = read_val_at(&mut lines, 1);
    instance.items_num = read_val_at(&mut lines, 3);
    instance.capacity = read_val_at(&mut lines, 3);
    instance.min_speed = read_val_at(&mut lines, 2);
    instance.max_speed = read_val_at(&mut lines, 2);
    instance.rent_ratio = read_val_at(&mut lines, 2);
    lines.nth(1);
    let mut coord_vec = vec![];
    for _ in 0..instance.dimension {
        coord_vec.push(read_coords(&lines.next().unwrap()));
    }
    instance.coords = coord_vec;
    lines.next();
    let mut item_vec = vec![];
    for _ in 0..instance.items_num {
        item_vec.push(read_item(&lines.next().unwrap()))
    }
    instance.items = item_vec;
    instance
}
