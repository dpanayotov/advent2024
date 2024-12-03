use std::fs;

pub fn day2_part2() {
    let mut input = read_content();
    let total = day2_calculation(&mut input);
    println!("total: {}", total);
}

pub fn day2_calculation(input: &mut Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for v in input.iter() {
        let mut unsafe_levels: Vec<_> = unsafe_detector(&v).map(|(i, invalid)| {
            return (i, invalid);
        }).collect(); // (index, [first, second])
        if unsafe_levels.len() > 0 {
            for (i, _) in unsafe_levels {
                if i > 0 {
                    let mut vec0 = v.clone();
                    vec0.remove(i - 1);
                    if unsafe_detector(&vec0).count() == 0 {
                        total += 1;
                        break;
                    }
                }

                let mut vec1 = v.clone();
                vec1.remove(i);
                if unsafe_detector(&vec1).count() == 0 {
                    total += 1;
                    break;
                }

                let mut vec2 = v.clone();
                vec2.remove(i + 1);
                if unsafe_detector(&vec2).count() == 0 {
                    total += 1;
                    break;
                }
            }
        } else {
            total += 1;
        }
    }
    total
}

fn unsafe_detector(v: &Vec<i32>) -> impl Iterator<Item=(usize, &[i32])> {
    let first = v[0];
    let second = v[1];
    let ascending = first < second;
    v.windows(2).enumerate().filter(move |(_, pair)| {
        let first = pair[0];
        let second = pair[1];
        let diff = (first - second).abs();
        return diff > 3 || (first >= second && ascending || first <= second && !ascending);
        ;
    })
}

pub fn day2_part1() {
    let input = read_content();

    let mut total = 0;
    for v in input.iter() {
        let detected_unsafe: Vec<_> = unsafe_detector(v).collect();
        let mut unsafe_levels_count: usize = detected_unsafe.len();
        if unsafe_levels_count == 0 { total += 1 }
    }
    println!("{}", total);
}

fn read_content() -> Vec<Vec<i32>> {
    let content = fs::read_to_string("inputs/day2.txt").expect("Could not read input");
    let mut input: Vec<Vec<i32>> = vec![];
    content.lines().for_each(|line| {
        let v: Vec<i32> = line.split_whitespace().map(|item| item.parse::<i32>().expect("not a number")).collect();
        input.push(v);
    });
    input
}
