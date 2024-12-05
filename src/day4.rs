use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

type Point = (usize, usize);
type Line = (Point, Point);

pub fn day4_part1() {
    let content = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
    let content = fs::read_to_string("inputs/day4.txt").expect("Could not read input");
    let total = day4_common(content, "XMAS", false, |diag| {});
    println!("Total: {}", total);
}

pub fn day4_part2() {
    let content =
        ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
    let content = fs::read_to_string("inputs/day4.txt").expect("Could not read input");

    let mut middle_points: HashMap<Point, usize> = HashMap::new();
    let diag_handler = |diagonals: Vec<Option<Line>>| {
        diagonals.iter().map(|line| {
            let l = line.unwrap();
            let midpoint = ((l.0.0 + l.1.0) / 2, (l.0.1 + l.1.1) / 2);
            return midpoint;
        }).for_each(|point| *middle_points.entry(point).or_insert(0) += 1)
    };
    day4_common(content, "MAS", true, diag_handler);
    println!("{:?}", middle_points);
    println!("Total intersections: {}", middle_points.iter().filter(|(_, &cnt)| cnt == 2).collect::<Vec<_>>().len());
}

fn day4_common<F>(content: String, search_str: &str, diag_only: bool, mut diag_handler: F) -> i32
    where F: FnMut(Vec<Option<Line>>) {
    let data: Vec<char> = search_str.chars().collect();
    let char_matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let max_height = char_matrix.len();
    let line_length = char_matrix[0].len();
    let mut total: i32 = 0;
    let first_char: char = data[0];
    let search_chars: &[char] = &data[1..];
    for row in 0..max_height {
        for col in 0..line_length {
            let curr_char = char_matrix[row][col];
            if curr_char == first_char {
                let diagonals = check_diagonals(&char_matrix, row, col, &search_chars);
                diag_handler(diagonals.clone());
                total += diagonals.len() as i32;
                if diag_only { continue; }
                total += vec![check_above(&char_matrix, row, col, &search_chars),
                              check_below(&char_matrix, row, col, &search_chars),
                              check_left(&char_matrix, row, col, &search_chars),
                              check_right(&char_matrix, row, col, &search_chars)].iter().filter(|result| result.is_some()).collect::<Vec<_>>().len() as i32;
            }
        }
    }
    return total;
}

fn check_above(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> { // start from row,col and walk upwards
    let search_len = search_chars.len();
    if row.checked_sub(search_len).is_none() { // to build XMAS we'll need 3 more letters and if we'll be outside no need to check
        return None;
    }

    for i in 0..search_len {
        if char_matrix[row - i - 1][col] != search_chars[i] {
            return None;
        }
    }
    return Some(((row, col), (row - search_len, col)));
}

fn check_below(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> {
    let search_len = search_chars.len();
    if row + search_len >= char_matrix.len() { // to build XMAS we'll need 3 more letters and if we'll be outside no need to check
        return None;
    }

    for i in 0..search_len {
        if char_matrix[row + i + 1][col] != search_chars[i] {
            return None;
        }
    }
    return Some(((row, col), (row + search_len, col)));
}

fn check_diagonals(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Vec<Option<Line>> { // start from row,col and walk diagonally
    return [check_top_right_diag(char_matrix, row, col, search_chars),
        check_top_left_diag(char_matrix, row, col, search_chars),
        check_bottom_left_diag(char_matrix, row, col, search_chars),
        check_bottom_right_diag(char_matrix, row, col, search_chars)].iter().filter(|result| result.is_some()).cloned().collect();
}

fn check_bottom_left_diag(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> {
    let search_len = search_chars.len();
    if row + search_len >= char_matrix[0].len() || col.checked_sub(search_len).is_none() { // increasing rows, decreasing columns
        return None;
    }

    for i in 0..search_len {
        if char_matrix[row + i + 1][col - i - 1] != search_chars[i] {
            return None;
        }
    }
    return Some(((row, col), (row + search_len, col - search_len)));
}

fn check_bottom_right_diag(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> {
    let search_len = search_chars.len();
    if row + search_len >= char_matrix[0].len() || col + search_len >= char_matrix[0].len() { // increasing rows, increasing columns
        return None;
    }

    for i in 0..search_len {
        if char_matrix[row + i + 1][col + i + 1] != search_chars[i] {
            return None;
        }
    }
    return Some(((row, col), (row + search_len, col + search_len)));
}

fn check_top_left_diag(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> {
    let search_len = search_chars.len();
    if row.checked_sub(search_len).is_none() || col.checked_sub(search_len).is_none() {
        return None;
    }

    for i in 0..search_len {
        if char_matrix[row - i - 1][col - i - 1] != search_chars[i] {
            return None;
        }
    }
    return Some(((row, col), (row - search_len, col - search_len)));
}

fn check_top_right_diag(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> {
    let search_len = search_chars.len();
    if row.checked_sub(search_len).is_none() || col + search_len >= char_matrix[0].len() { // decreasing rows, but increasing columns
        return None;
    }
    for i in 0..search_len {
        if char_matrix[row - i - 1][col + i + 1] != search_chars[i] {
            return None;
        }
    }
    return Some(((row, col), (row - search_len, col + search_len)));
}

fn check_left(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> { // start from row,col and walk left
    let search_len = search_chars.len();
    if col.checked_sub(search_len).is_none() { // to build XMAS we'll need 3 more letters and if we'll be outside no need to check
        return None;
    }
    for i in 0..search_len {
        if char_matrix[row][col - i - 1] != search_chars[i] {
            return None;
        }
    }
    return Some(((row, col), (row, col - search_len)));
}

fn check_right(char_matrix: &Vec<Vec<char>>, row: usize, col: usize, search_chars: &[char]) -> Option<Line> { // start from row,col and walk right
    let search_len = search_chars.len();
    if col + search_len >= char_matrix[0].len() { // to build XMAS we'll need 3 more letters and if we'll be outside no need to check
        return None;
    }
    for i in 0..search_len {
        if char_matrix[row][col + i + 1] != search_chars[i] {
            return None;
        }
    }

    return Some(((row, col), (row, col + search_len)));
}
