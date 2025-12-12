use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_1.txt").expect("Error reading the file")
}

pub fn first() {
    let code = read_file().lines().map(|s| {
        let n: i32 = s[1..].parse().unwrap();
        if s[0..1] == *"L" {
            -n
        } else {
            n
        }
    }).fold((50, 0), |acc, turn| {
        let mut pos = acc.0 + turn;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            (pos, acc.1 + 1)
        } else {
            (pos, acc.1)
        }
    }).1;
    println!("{}", code);
}

pub fn second() {
    let code = read_file().lines().map(|s| {
        let n: i32 = s[1..].parse().unwrap();
        if s[0..1] == *"L" {
            -n
        } else {
            n
        }
    }).fold((50, 0), |(prev_pos, zero_count), turn| {
        let pos = prev_pos + turn;
        if turn > 0 {
            (pos % 100, zero_count + pos / 100)
        } else {
            let wrap = (100 - prev_pos) % 100 - turn;
            (pos.rem_euclid(100), zero_count + wrap / 100)
        }
    }).1;
    println!("{}", code);
}