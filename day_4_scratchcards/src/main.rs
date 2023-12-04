use std::{fs, collections::HashSet};

struct InputData {
    content: Vec<u8>,
    left_start: usize,
    right_start: usize,
    rows: usize,
    row_length: usize
}

fn get_next_number(content: &Vec<u8>, start: usize) -> usize {
    let mut total = 0;
    for &character in &content[start..(start + 2)] {
        if character != b' ' {total = (total * 10) + (character as usize - 48);}
    }
    return total;
}

fn get_left_numbers(source: &InputData, row_start: usize) -> HashSet<usize> {
    let InputData{content, left_start, right_start, ..} = source;
    let mut i  = row_start + *left_start;
    let mut numbers = HashSet::new();
    while i - row_start < *right_start - 2 {
        numbers.insert(get_next_number(&content, i));
        i += 3;
    }
    return numbers;
}

fn get_right_numbers(source: &InputData, row_start: usize) -> HashSet<usize> {
    let InputData{content, right_start, row_length,..} = source;
    let mut i = row_start + *right_start;
    let mut numbers = HashSet::new();
    while i - row_start < *row_length - 1 {
        numbers.insert(get_next_number(&content, i));
        i += 3;
    }
    return numbers;
}

fn pt_1(source: InputData) -> usize {
    let mut matches;
    let mut total = 0;
    for row in 0..source.rows {
        matches = 0;
        let right_numbers = get_right_numbers(&source, source.row_length * row);
        for item in get_left_numbers(&source, source.row_length * row) {
            if right_numbers.contains(&item) {
                matches += 1;
            }
        }
        if matches > 0 {
            total += 1 << (matches - 1);
        }
    }
    return total;
}

fn main() {
    let content = fs::read("input.txt").expect("failed to read input");
    let mut i = 0;
    while content[i] != b':' {
        i += 1;
    }
    i += 2;
    let left_start = i;
    while content[i] != b'|' {
        i += 3
    }
    i += 2;
    let right_start = i;
    while content[i] != b'\n' {
        i += 3
    }
    i += 1;
    let rows = content.len() / i;
    println!("{}", pt_1(InputData{content, left_start, right_start, rows, row_length: i}));
}
