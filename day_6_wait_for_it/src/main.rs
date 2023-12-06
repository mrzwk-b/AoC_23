use std::{env, fs};

fn get_times(source: &Vec<u8>) -> Vec<usize> {
    let mut times = Vec::new();
    let mut i = 0;
    while source[i] != b'\n' {
        while !(48..58).contains(&source[i]) {i += 1;}
        let mut total = 0;
        while (48..58).contains(&source[i]) {
            total = (total * 10) + (source[i] as usize - 48);
            i += 1;
        }
        times.push(total);
    }
    return times;
}

fn get_records(source: &Vec<u8>) -> Vec<usize> {
    let mut records = Vec::new();
    let mut i = source.len() - 2;
    while source[i] != b':' {
        let mut digits = Vec::new();
        while (48..58).contains(&source[i]) {
            digits.push(source[i] as usize - 48);
            i -= 1;
        }
        let mut total = 0;
        for _ in 0..digits.len() {
            total = (total * 10) + digits.pop().unwrap();
        }
        records.push(total);
        while source[i] == b' ' {i -= 1;}
    }
    records.reverse();
    return records;
}

fn compute_buttons(times: Vec<usize>, records: Vec<usize>) -> usize {
    let mut total = 1;
    for i in 0..times.len() {
        let s = (((times[i] * times[i]) as f64 / 4.) - (records[i] as f64)).sqrt();
        let t = times[i] as f64 / 2.;
        total *= (t + s) as usize - (t - s) as usize - (s == (s as usize as f64)) as usize;
    }
    return total;
}

fn pt_1(source: &Vec<u8>) -> usize {
    return compute_buttons(get_times(&source), get_records(&source));
}

fn get_time(source: &Vec<u8>) -> Vec<usize> {
    let mut times = Vec::new();
    let mut total = 0;
    let mut i = 0;
    while source[i] != b'\n' {
        while !(48..58).contains(&source[i]) {i += 1;}
        while (48..58).contains(&source[i]) {
            total = (total * 10) + (source[i] as usize - 48);
            i += 1;
        }
    }
    times.push(total);
    return times;
}

fn get_record(source: &Vec<u8>) -> Vec<usize> {
    let mut records = Vec::new();
    let mut digits = Vec::new();
    let mut i = source.len() - 2;
    while source[i] != b':' {
        while (48..58).contains(&source[i]) {
            digits.push(source[i] as usize - 48);
            i -= 1;
        }
        while source[i] == b' ' {i -= 1;}
    }
    let mut total = 0;
    for _ in 0..digits.len() {
        total = (total * 10) + digits.pop().unwrap();
    }
    records.push(total);
    return records;
}

fn pt_2(source: &Vec<u8>) -> usize {
    return compute_buttons(get_time(&source), get_record(&source));
}

// code designed for LF .txt input
fn main() {
    let args: Vec<String> = env::args().collect();
    let source = fs::read(&args[1]).expect("failed to read input");
    
    println!("{}", pt_1(&source));
    println!("{}", pt_2(&source));
}
