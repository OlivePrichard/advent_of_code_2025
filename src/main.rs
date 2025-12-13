#![allow(dead_code)]

mod days;

use days::day_2::*;

fn time(function: fn()) {
    let start = std::time::Instant::now();
    function();
    let stop = std::time::Instant::now();
    let duration = stop - start;
    if duration.as_millis() > 0 {
        println!("{} ms", duration.as_millis());
    } else if duration.as_micros() > 0 {
        println!("{} us", duration.as_micros());
    } else {
        println!("{} ns", duration.as_nanos());
    }
}

fn main() {
    print!("First: ");
    time(first);
    print!("Second: ");
    time(second);
}
