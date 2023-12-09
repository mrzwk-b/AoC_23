use std::{env, fs, collections::HashMap, vec};

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
fn get_nodes<'a>(source: &Vec<u8>, start: usize) -> HashMap<String, [String; 2]> {
    let mut nodes = HashMap::new();
    for i in 0..((source.len() - (start + 1)) / 17) {
        let mut current = String::new();
        let mut left = String::new();
        let mut right = String::new();
        for j in 0..3 {
            current.push(source[start + (i*17) + j] as char);
            left.push(source[start + (i*17) + 7 + j] as char);
            right.push(source[start + (i*17) + 12 + j] as char);
        }
        nodes.insert(current, [left, right]);
    } 
    return nodes;
}

fn pt_1(source: &Vec<u8>) -> usize {
    let directions = get_directions(&source);
    let nodes = get_nodes(&source, directions.len() + 2);

    let mut current_node = String::from("AAA");
    let mut total = 0;
    while current_node != String::from("ZZZ") {
        println!("{current_node}");
        current_node = nodes.get(&current_node).unwrap()[directions[total % directions.len()] as usize].clone();
        total += 1;
    }
    return total;
}

// code intended for LF .txt input
fn main() {
    let source = fs::read(&env::args().collect::<Vec<String>>()[1]).expect("failed to read file");

    println!("{}", pt_1(&source));
}
