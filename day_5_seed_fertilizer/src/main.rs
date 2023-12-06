use std::{fs, env, collections::HashMap, usize, ops::Range};

// returned tuple formatted as (destination start, source start, range size) just like in document
fn parse_line(almanac: &Vec<u8>, index: &mut usize) -> [usize; 3] {
    let mut line_values: [usize; 3] = [0, 0, 0];
    for i in 0..3 {
        let mut total = 0;
        while (48..58).contains(&almanac[*index]) {
            total = (total * 10) + (almanac[*index] as usize - 48);
            *index += 1;
        }
        line_values[i] = total;
        *index += 1;
    }
    return line_values;
}

fn next_map(almanac: &Vec<u8>, index: &mut usize) -> Option<HashMap<Range<usize>, Range<usize>>> {
    while *index < almanac.len() && !(48..58).contains(&almanac[*index]) {*index += 1;}
    let mut map = HashMap::new();
    while *index < almanac.len() && (48..58).contains(&almanac[*index]) {
        let [destination_start, source_start, range_size] = parse_line(&almanac, index);
        map.insert(source_start..source_start + range_size, destination_start..destination_start + range_size);
    }
    if map.len() == 0 {
        return None;
    }
    return Some(map);
}

// doesn't actually need to take index but it's more convenient for the rest of the program that it advances
fn get_seeds(almanac: &Vec<u8>, index: &mut usize) -> Vec<usize> {
    while !(48..58).contains(&almanac[*index]) {*index += 1;}
    let mut seeds = Vec::new();
    while almanac[*index] != b'\n' {
        let mut total = 0;
        while (48..58).contains(&almanac[*index]) {
            total = (total * 10) + (almanac[*index] as usize - 48);
            *index += 1;
        }
        seeds.push(total);
        *index += 1;
    }
    return seeds;
}

fn pt_1(almanac: Vec<u8>) -> usize {
    let mut index = 0;
    let mut prev_data = get_seeds(&almanac, &mut index);
    let mut next_data = prev_data.clone();
    while let Some(map) = next_map(&almanac, &mut index) {
        for i in 0..prev_data.len() {
            for r in map.keys() {
                if r.contains(&prev_data[i]) {
                    next_data[i] = map.get(r).unwrap().start + (prev_data[i] - r.start);
                }
            }
        }
        prev_data = next_data.clone();
    }
    return *prev_data.iter().min().unwrap();
}

// code designed for LF .txt input
fn main() {
    let args: Vec<String> = env::args().collect();
    let almanac = fs::read(&args[1]).expect("failed to read input");
    
    println!("{}", pt_1(almanac));
}
