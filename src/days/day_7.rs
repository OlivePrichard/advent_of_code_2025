use std::{collections::{HashMap, hash_map::Entry}, fs};

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_7.txt").expect("Error reading the file")
}

pub fn first() {
    let mut lines = read_file().lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let mut splits = 0;
    for i in 1..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i-1][j] != '|' && lines[i-1][j] != 'S' {
                continue;
            }
            if lines[i][j] == '^' {
                lines[i][j-1] = '|';
                lines[i][j+1] = '|';
                splits += 1;
            } else {
                lines[i][j] = '|';
            }
        }
    }
    println!("{}", splits);
}

fn count_timelines(i: usize, j: usize, manifold: &Vec<Vec<char>>, memoize: &mut HashMap<(usize, usize), u64>) -> u64 {
    if i == manifold.len() - 1 {
        return 1;
    }
    let key = (i, j);
    if let Entry::Occupied(val) = memoize.entry(key) {
        return *val.get();
    }
    let i = i + 1;
    let count = if manifold[i][j] == '^' {
        count_timelines(i, j-1, manifold, memoize) + count_timelines(i, j+1, manifold, memoize)
    } else {
        count_timelines(i, j, manifold, memoize)
    };
    _ = memoize.insert(key, count);
    count
}

pub fn second() {
    let lines = read_file().lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let j = lines[0].iter().position(|&c| c == 'S').unwrap();
    let timelines = count_timelines(0, j, &lines, &mut HashMap::new());
    println!("{}", timelines);
}