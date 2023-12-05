use std::{fs, env, ops::Range};

fn parse_line(data: &Vec<u8>, mut start_index: &usize) -> (Range<usize>, Range<usize>) {
    return (0..1, 0..1);
}

fn remap(data: &Vec<u8>, start_data: &Vec<usize>, mut index: &usize) -> Vec<usize> {
    let mut end_data = start_data.clone();
    while data[*index] != b'\n' {
        let map = parse_line(&data, &mut index);
        for j in 0..start_data.len() {
            if map.0.contains(&start_data[j]) {
                
            }
        }
    }
    return end_data
}

fn pt_1(data: Vec<u8>) -> usize {
    let mut table: [Vec<usize>; 8];

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(&args[1]).expect("failed to read input");
    
    println!("{}", pt_1(data));
}
