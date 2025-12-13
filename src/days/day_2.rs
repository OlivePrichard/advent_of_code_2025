use std::fs;
use regex::Regex;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_2.txt").expect("Error reading the file")
}

fn next_doubled(num: u64) -> u64 {
    if num == 0 {
        return 11;
    }
    // println!("{}", num);
    let digits = num.ilog10() + 1;
    // println!("digits: {}", digits);
    let half = 10u64.pow(digits / 2);
    // println!("half: {}", half);
    if digits % 2 == 1 {
        return half * half * 10 + half;
    }
    let left = num / half;
    let right = num % half;
    if left > right {
        return left * half + left;
    }
    let new_left = left + 1;
    if new_left.ilog10() != left.ilog10() {
        new_left * half * 10 + new_left
    } else {
        new_left * half + new_left
    }
}

fn check_repeated(s: &str, n: usize) -> bool {
    let steps = s.len() / n;
    for i in 0..steps {
        if s[(i*n)..(i*n+n)] != s[0..n] {
            return false;
        }
    }
    true
}

fn check_num(num: u64) -> bool {
    let digits = num.to_string();
    for i in 1..=(digits.len()/2) {
        if digits.len() % i == 0 && check_repeated(&digits, i) {
            return true;
        }
    }
    false
}

pub fn first() {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let total: u64 = re.captures_iter(&read_file()).map(|c| {
        let (_, [a, b]) = c.extract();
        // println!("{} - {}", a, b);
        let range = a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap();
        // println!("{:?}", range);
        let mut acc = 0;
        let mut num = next_doubled(range.start() - 1);
        while range.contains(&num) {
            // println!("{}", num);
            acc += num;
            num = next_doubled(num);
        }
        acc
    }).sum();
    println!("{}", total);
    // println!("{}", next_doubled(3434061167 - 1));
}

pub fn second() {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let total: u64 = re.captures_iter(&read_file()).map(|c| {
        let (_, [a, b]) = c.extract();
        let range = a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap();
        // println!("{:?}", range);
        let mut acc = 0;
        for i in range {
            if check_num(i) {
                acc += i;
                // println!("{}", i);
            }
        }
        acc
    }).sum();
    println!("{}", total);
}