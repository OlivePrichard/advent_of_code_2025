use std::fs;

use regex::Regex;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_6.txt").expect("Error reading the file")
}

pub fn first() {
    let num_re = Regex::new(r"(\d+)\s*").unwrap();
    let op_re = Regex::new(r"(\S)\s*").unwrap();
    let file_string = read_file();
    let ops = op_re.captures_iter(file_string.lines().last().unwrap()).map(|c| {
        let (_, [op]) = c.extract();
        op.chars().next().unwrap()
    }).collect::<Vec<_>>();
    let total: u64 = file_string.lines().rev().skip(1).map(|line| {
        num_re.captures_iter(line).map(|c| {
            let (_, [num]) = c.extract();
            num.parse::<u64>().unwrap()
        }).collect::<Vec<u64>>()
    }).fold(ops.iter().map(|&op| if op == '*' {1} else {0}).collect::<Vec<u64>>(), |acc, v| {
        ops.iter().zip(acc.iter().zip(v)).map(|(&op, (&a, b))| if op == '*' {a*b} else {a+b}).collect::<Vec<u64>>()
    }).into_iter().sum();
    println!("{}", total);
}

pub fn second() {
    let op_re = Regex::new(r"(\S)\s*").unwrap();
    let file_string = read_file();
    let ops = op_re.captures_iter(file_string.lines().last().unwrap()).map(|c| {
        let (_, [op]) = c.extract();
        op.chars().next().unwrap()
    }).collect::<Vec<_>>();
    let chars = file_string.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut nums = Vec::new();
    for i in 0..chars[0].len() {
        let mut s = String::new();
        for j in 0..chars.len()-1 {
            s.push(chars[j][i]);
        }
        nums.push(s.trim().to_string());
    }
    nums.push(String::new());
    let mut j = 0;
    let mut sum = 0;
    let mut acc = 1;
    for num in nums {
        if num.len() == 0 {
            sum += acc;
            if ops[j] == '+' {
                sum -= 1;
            }
            j += 1;
            acc = 1;
            continue;
        }
        let n: u64 = num.parse().unwrap();
        if ops[j] == '+' {
            acc += n;
        } else {
            acc *= n;
        }
    }
    println!("{}", sum);
}