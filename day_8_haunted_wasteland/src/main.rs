use std::{env, fs, collections::HashMap};

// left is false, right is true
fn get_directions(source: &Vec<u8>) -> Vec<bool> {
    let mut directions = vec![];
    let mut i = 0;
    while source[i] != b'\n' {
        directions.push(source[i] == b'R');
        i += 1;
    }
    return directions;
}
fn get_nodes(source: &Vec<u8>, start: usize) -> HashMap<&str, [&str; 2]> {
    let mut nodes = HashMap::new();
    for i in 0..((source.len() - (start + 1)) / 17) {
        nodes.insert(
            source[start + i*17 .. start + (i*17)+3],
            [
                source[start + (i*17)+7 .. start + (i*17)+10, 
                start + (i*17)+12 .. start + (i*17)+15],
            ]
        );
    }
        
    return nodes;
}

fn pt_1(source: &Vec<u8>) -> usize {
    let directions = get_directions(&source);
    let nodes = get_nodes(&source, directions.len() + 2);

    let mut current_node = "AAA";
    let mut total = 0;
    while current_node != "ZZZ" {
        current_node = nodes.get(current_node).unwrap()[directions[total % directions.len()] as usize];
        total += 1;
    }
    return total;
}

// code intended for LF .txt input
fn main() {
    let source = fs::read(&env::args().collect::<Vec<String>>()[1]).expect("failed to read file");

    println!("{}", pt_1(&source));
}
