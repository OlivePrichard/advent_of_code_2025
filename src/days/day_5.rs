use std::fs;

use regex::Regex;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_5.txt").expect("Error reading the file")
}

pub fn first() {
    let range_re = Regex::new(r"(?m)^(\d+)-(\d+)$").unwrap();
    let value_re = Regex::new(r"(?m)^(\d+)$").unwrap();
    let file_string = read_file();
    let ranges = range_re.captures_iter(&file_string).map(|capture| {
        let (_, [first, second]) = capture.extract();
        first.parse::<u64>().unwrap()..=second.parse().unwrap()
    }).collect::<Vec<_>>();
    let fresh_items = value_re.captures_iter(&file_string).map(|capture| {
        let (_, [val]) = capture.extract();
        val.parse::<u64>().unwrap()
    }).filter(|n| {
        ranges.iter().any(|r| r.contains(n))
    }).count();
    println!("{}", fresh_items);
}

pub fn second() {
    let range_re = Regex::new(r"(?m)^(\d+)-(\d+)$").unwrap();
    let file_string = read_file();
    let mut ranges = range_re.captures_iter(&file_string).map(|capture| {
        let (_, [first, second]) = capture.extract();
        first.parse::<u64>().unwrap()..(second.parse::<u64>().unwrap()+1)
    }).collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.start);
    let mut merged_ranges = Vec::new();
    let mut previous_range = ranges[0].clone();
    for range in ranges.into_iter().skip(1) {
        if range.start <= previous_range.end {
            previous_range.end = previous_range.end.max(range.end);
            continue;
        }
        merged_ranges.push(previous_range);
        previous_range = range;
    }
    merged_ranges.push(previous_range);
    let fresh_ids: u64 = merged_ranges.into_iter().map(|r| r.end - r.start).sum();
    println!("{}", fresh_ids);
}