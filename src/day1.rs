use std::collections::HashMap;
use std::fs;
use std::iter::zip;

pub fn day1_part1() {
    let content = fs::read_to_string("inputs/day1.txt").expect("Could not read input");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    content.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>().expect("not a number"));
        right.push(parts[1].parse::<i32>().expect("Not a number"));
    });
    left.sort();
    right.sort();

    let mut sum: i32 = 0;
    for (left_val, right_val) in zip(left.iter(), right.iter()) {
        sum += (right_val - left_val).abs();
    }
    println!("Total distance is {}", sum)
}

pub fn day1_part2() {
    let content = fs::read_to_string("inputs/day1.txt").expect("Could not read input");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    content.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>().expect("not a number"));
        right.push(parts[1].parse::<i32>().expect("Not a number"));
    });
    let mut right_values: HashMap<i32, i32> = HashMap::new();
    right.iter().for_each(|value| *right_values.entry(*value).or_default() += 1);
    let mut sum: i32 = 0;
    left.iter().for_each(|value| {
        sum += value * (right_values.get(value)).unwrap_or(&0);
    });
    println!("{}", sum);
}