use std::{collections::HashSet, fs};

use regex::Regex;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_8.txt").expect("Error reading the file")
}

fn get_set<'a>(i: usize, sets: impl Iterator<Item=&'a HashSet<usize>>) -> Option<usize> {
    for (j, set) in sets.enumerate() {
        if set.contains(&i) {
            return Some(j);
        }
    }
    None
}

pub fn first() {
    let re = Regex::new(r"(?m)^(\d+),(\d+),(\d+)$").unwrap();
    let points: Vec<(i64, i64, i64)> = re.captures_iter(&read_file()).map(|c| {
        let (_, [x, y, z]) = c.extract();
        (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
    }).collect();
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut pairs: Vec<(usize, usize)> = (0..points.len()).flat_map(|larger| (0..larger).map(move |smaller| (smaller, larger))).collect();
    pairs.sort_by_key(|&(i, j)| {
        let (x1, y1, z1) = points[i];
        let (x2, y2, z2) = points[j];
        let (dx, dy, dz) = (x1-x2, y1-y2, z1-z2);
        dx*dx+dy*dy+dz*dz
    });
    for &(i, j) in pairs[0..1000].into_iter() {
        match (get_set(i, circuits.iter()), get_set(j, circuits.iter())) {
            (Some(a), Some(b)) => if a != b {
                let small = a.min(b);
                let large = a.max(b);
                let second = circuits.remove(large);
                circuits[small].extend(second);
            }
            (Some(a), None) => {
                circuits[a].insert(j);
            }
            (None, Some(b)) => {
                circuits[b].insert(i);
            }
            (None, None) => {
                circuits.push(HashSet::from_iter([i, j]));
            }
        }
    }
    circuits.sort_by_key(|set| -(set.len() as i32));
    let res = circuits[0].len() * circuits[1].len() * circuits[2].len();
    println!("{}", res);
}

pub fn second() {
    let re = Regex::new(r"(?m)^(\d+),(\d+),(\d+)$").unwrap();
    let points: Vec<(i64, i64, i64)> = re.captures_iter(&read_file()).map(|c| {
        let (_, [x, y, z]) = c.extract();
        (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
    }).collect();
    let mut pairs: Vec<(usize, usize)> = (0..points.len()).flat_map(|larger| (0..larger).map(move |smaller| (smaller, larger))).collect();
    let mut circuits = Vec::from_iter((0..points.len()).map(|n| HashSet::from([n])));
    pairs.sort_by_key(|&(i, j)| {
        let (x1, y1, z1) = points[i];
        let (x2, y2, z2) = points[j];
        let (dx, dy, dz) = (x1-x2, y1-y2, z1-z2);
        dx*dx+dy*dy+dz*dz
    });
    let mut res = 0;
    for (i, j) in pairs {
        let (a, b) = (get_set(i, circuits.iter()).unwrap(), get_set(j, circuits.iter()).unwrap());
        if a == b {
            continue;
        }
        let other = circuits.remove(a.max(b));
        circuits[a.min(b)].extend(other);
        if circuits.len() == 1 {
            res = points[i].0 * points[j].0;
            break;
        }
    }
    println!("{}", res);
}