use std::{fs, env};

fn remap(&data: Vec<u8>, &start_data: Vec<usize>, start_index: usize) -> Vec<usize> {

}

fn pt_1(data: Vec<u8>) -> usize {
    let table: [Vec<usize>; 8];

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(&args[1]).expect("failed to read input");
    
    println!("{}", pt_1(data));
}
