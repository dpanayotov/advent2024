#![feature(int_roundings)]
#![feature(iter_collect_into)]
extern crate core;

use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let mut result: Vec<Vec<i32>> = Vec::new();
    result.push(vec![7, 6, 4, 2, 1]); // safe
    result.push(vec![1, 2, 7, 8, 9]); // unsafe
    result.push(vec![9, 7, 6, 2, 1]); // unsafe
    result.push(vec![1, 3, 2, 4, 5]); // unsafe
    result.push(vec![8, 6, 4, 4, 1]); // unsafe
    result.push(vec![1, 3, 6, 7, 9]); // unsafe

    // result.push(vec![46, 46, 49, 55, 57, 59, 60, 59]); // unsafe
    // result.push(vec![37, 35, 33, 33, 33]); //unsafe
    // day2::day2_calculation(&mut result);
    // measure_execution(day2::day2_part2);
    // measure_execution(day3::day3_part1);
    // measure_execution(day3::day3_part2);
    measure_execution(day4::day4_part2);
}

fn measure_execution(x: fn()) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    x();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Took {:?}", end.sub(start));
}