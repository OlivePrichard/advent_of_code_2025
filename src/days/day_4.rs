use std::fs;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_4.txt").expect("Error reading the file")
}

pub fn first() {
    let mut rolls: Vec<Vec<char>> = read_file().lines().map(|line| {
        line.chars().collect()
    }).collect();
    let offsets = [
        ( 1,  0),
        (-1,  0),
        ( 0,  1),
        ( 0, -1),
        ( 1,  1),
        (-1, -1),
        (-1,  1),
        ( 1, -1),
    ];
    let mut count = 0;
    for i in 0..rolls.len() {
        for j in 0..rolls[i].len() {
            if rolls[i][j] == '.' {
                continue;
            }
            let mut paper = 0;
            for offset in offsets {
                let (i, j) = (i as i32 + offset.0, j as i32 + offset.1);
                if !(0..rolls.len() as i32).contains(&i) || !(0..rolls[i as usize].len() as i32).contains(&j) {
                    continue;
                }
                if rolls[i as usize][j as usize] == '@' || rolls[i as usize][j as usize] == 'x' {
                    paper += 1;
                }
            }
            if paper < 4 {
                count += 1;
                rolls[i][j] = 'x';
            }
        }
    }
    // for i in 0..rolls.len() {
    //     for j in 0..rolls[i].len() {
    //         print!("{}", rolls[i][j]);
    //     }
    //     println!();
    // }
    println!("{}", count);
}

pub fn second() {
    let mut rolls: Vec<Vec<char>> = read_file().lines().map(|line| {
        line.chars().collect()
    }).collect();
    let offsets = [
        ( 1,  0),
        (-1,  0),
        ( 0,  1),
        ( 0, -1),
        ( 1,  1),
        (-1, -1),
        (-1,  1),
        ( 1, -1),
    ];
    let mut total_count = 0;
    loop {
        let mut count = 0;
        for i in 0..rolls.len() {
            for j in 0..rolls[i].len() {
                if rolls[i][j] == '.' {
                    continue;
                }
                let mut paper = 0;
                for offset in offsets {
                    let (i, j) = (i as i32 + offset.0, j as i32 + offset.1);
                    if !(0..rolls.len() as i32).contains(&i) || !(0..rolls[i as usize].len() as i32).contains(&j) {
                        continue;
                    }
                    if rolls[i as usize][j as usize] == '@' || rolls[i as usize][j as usize] == 'x' {
                        paper += 1;
                    }
                }
                if paper < 4 {
                    count += 1;
                    rolls[i][j] = 'x';
                }
            }
        }
        if count == 0 {
            break;
        }
        total_count += count;
        for i in 0..rolls.len() {
            for j in 0..rolls[i].len() {
                if rolls[i][j] == 'x' {
                    rolls[i][j] = '.';
                }
            }
        }
    }
    println!("{}", total_count);
}