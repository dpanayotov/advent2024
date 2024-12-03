use std::fs;
use regex::Regex;

pub fn day3_part1() {
    let content = fs::read_to_string("inputs/day3.txt").expect("Could not read input");
    let sum = extract_sum(&content);
    println!("{}", sum);
}

fn extract_sum(content: &String) -> i64 {
    let mut total: i64 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let line = &content.replace("\n", "");
    re.captures_iter(line).for_each(|c| {
        let (_, [first, second]) = c.extract();
        total += first.parse::<i64>().expect("first is not a number") * second.parse::<i64>().expect("second is not a number");
    });
    return total;
}

pub fn day3_part2() {
    let mut content = fs::read_to_string("inputs/day3.txt").expect("Could not read input");

    let line = &format!("do(){}don't()", &content.replace("\n", ""));
    let re_boundary = Regex::new(r"do\(\)(?<between>.*?)don't\(\)").unwrap();
    let mut total: i64 = 0;
    re_boundary.captures_iter(line).for_each(|c| {
        let s = c.name("between").map_or("", |m| m.as_str());
        total += extract_sum(&s.to_string());
    });
    println!("{}", total);
}