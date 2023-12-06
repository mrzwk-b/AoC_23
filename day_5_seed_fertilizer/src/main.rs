use std::{fs, env, collections::{HashMap, BTreeMap}, usize, ops::Range};

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
fn get_seeds_pt_1(almanac: &Vec<u8>, index: &mut usize) -> Vec<usize> {
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

fn pt_1(almanac: &Vec<u8>) -> usize {
    let mut index = 0;
    let mut prev_data = get_seeds_pt_1(&almanac, &mut index);
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

fn get_seeds_pt_2(almanac: &Vec<u8>, index: &mut usize) -> Vec<Range<usize>> {
    while !(48..58).contains(&almanac[*index]) {*index += 1;}
    let mut seeds = Vec::new();
    while almanac[*index] != b'\n' {
        let mut range_start = 0;
        for i in 0..2 {    
            let mut total = 0;
            while (48..58).contains(&almanac[*index]) {
                total = (total * 10) + (almanac[*index] as usize - 48);
                *index += 1;
            }
            if i == 0 {
                range_start = total
            }
            else {
                seeds.push(range_start..range_start + total);
            }
            *index += 1;
        }
    }
    return seeds;
}

fn remap_ranges(in_data: Vec<Range<usize>>, map: HashMap<Range<usize>, Range<usize>>) -> Vec<Range<usize>> {
    let mut out_data = Vec::new();
    for i in 0..in_data.len() {
        let in_range = in_data[i].clone();
        // out_ranges will be represented as a map of break points onto new values
        let mut out_ranges = BTreeMap::new();
        out_ranges.insert(in_range.start, in_range.start);
        out_ranges.insert(in_range.end, if i + 1 < in_data.len() {in_data[i+1].start} else {in_range.end});
        for r in map.keys() {
            if r.start < in_range.end && r.end > in_range.start {
                if r.start > in_range.start{
                    if out_ranges.contains_key(&r.start) {out_ranges.remove(&r.start);}
                    out_ranges.insert(r.start, map.get(r).unwrap().start);
                }
                else {
                    out_ranges.remove(&in_range.start);
                    out_ranges.insert(in_range.start, map.get(r).unwrap().start + in_range.start- r.start);
                }
                if r.end < in_range.end {
                    if !out_ranges.contains_key(&r.end) {out_ranges.insert(r.end, r.end);}
                }
                else {
                    out_ranges.remove(&in_range.end);
                    out_ranges.insert(in_range.end, map.get(r).unwrap().end + r.end - in_range.end);
                }
            }
        }
        for i in 1..out_ranges.len() {
            out_data.push(
                *out_ranges.get(out_ranges.keys().collect::<Vec<_>>()[i-1]).unwrap()..
                *out_ranges.get(out_ranges.keys().collect::<Vec<_>>()[i]).unwrap()
            );
        }
    }
    return out_data;
}

fn pt_2(almanac: &Vec<u8>) -> usize {
    let mut index = 0;
    let mut data = get_seeds_pt_2(&almanac, &mut index);
    while let Some(map) = next_map(&almanac, &mut index) {
        data = remap_ranges(data, map);
    }
    return {
        let mut i = data.len();
        let mut min = usize::MAX;
        for j in 0..data.len() {
            if data[j].start < min {
                i = j;
                min = data[j].start;
            }
        }
        data[i].start
    };
}

// code designed for LF .txt input
fn main() {
    let args: Vec<String> = env::args().collect();
    let almanac = fs::read(&args[1]).expect("failed to read input");
    
    println!("{}", pt_1(&almanac));
    println!("{}", pt_2(&almanac));
}
