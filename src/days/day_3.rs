use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_3.txt").expect("Error reading the file")
}

pub fn first() {
    let output: u32 = read_file().lines().map(|line| {
        let bats: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        // println!("{:?}", bats);
        let &first = bats[0..bats.len()-1].iter().max().unwrap();
        let i = bats.iter().position(|&n| n==first).unwrap() as usize;
        // println!("{}", i);
        let ret = 10 * first + bats[i+1..bats.len()].iter().max().unwrap();
        // println!("{}", ret);
        ret
    }).sum();
    println!("{}", output);
}

pub fn second() {
    let output: u64 = read_file().lines().map(|line| {
        let bats: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).enumerate().collect();
        let mut acc = 0;
        let mut index = 0;
        for i in 0..12 {
            let exclude = 11-i;
            let search_slice = &bats[index..bats.len()-exclude];
            let &(j, max) = search_slice.iter().rev().max_by_key(|(_, n)| n).unwrap();
            index = j + 1;
            acc *= 10;
            acc += max as u64;
        }
        acc
    }).sum();
    println!("{}", output);
}