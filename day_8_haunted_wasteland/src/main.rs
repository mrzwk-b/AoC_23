use std::{env, fs, collections::{HashMap, HashSet}, vec};

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
fn get_nodes(source: &Vec<u8>, start: usize) -> HashMap<String, [String; 2]> {
    let mut nodes = HashMap::new();
    for i in 0..((source.len() - start) / 17) {
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
        current_node = nodes.get(&current_node).unwrap()[directions[total % directions.len()] as usize].clone();
        total += 1;
    }
    return total;
}

struct Path<'a> {
    // ignores the concept of nodes having two directions 
    // in favor of prefixing the direction you'll take at the end of the node name
    seen: &'a mut HashMap<&'a String, usize>,
    index: &'a mut usize,
    in_loop: &'a mut bool,
    loop_start: &'a mut String,
    loop_length: &'a mut usize,
    endpoints: &'a mut HashSet<usize>
}
impl<'a> Path<'a> {
    fn add(&mut self, value: &'a String) {
        *self.index += 1;
        // if we're done looping
            // do nothing
        if !(*self.in_loop || *self.loop_length == 0) {
            return;
        }
        // if we're not done looping
            // check if we're done and increment counter
        else if *self.in_loop {
            if value == self.loop_start {
                *self.in_loop = false;
            }
            *self.loop_length += 1;
        }
        // if we haven't even started looping
            // check if we just did
            // else record another node
        else {
            if self.seen.contains_key(value) {
                self.endpoints.clear();
                *self.in_loop = true;
            }
            else {
                self.seen.insert(value, *self.index);
            }
        }
        // check if we're on an endpoint
        if value.ends_with('Z') {
            self.endpoints.insert(*self.index);
        }
    }
    fn at_end(&self) -> bool {
        return 
            self.endpoints.contains(&
                if *self.loop_length != 0 {
                    let x: usize = (*self.index - self.seen.get(self.loop_start).unwrap()) % *self.loop_length;
                    x
                }
                else {
                    *self.index
                }
            )
        ;
    }
}

fn pt_2(source: &Vec<u8>) -> usize {
    let directions = get_directions(&source);
    let nodes = get_nodes(&source, directions.len() + 2);

    let mut current_nodes = Vec::new();

    let mut seen = HashMap::new();
    let mut index = 0;
    let mut in_loop = false;
    let mut loop_start = String::new();
    let mut loop_length = 0;
    let mut endpoints = HashSet::new();
    for n in nodes.iter() {
        if n.0.ends_with('A') {

            let p = Path{
                seen: &mut seen.clone(),
                index: &mut index,
                in_loop: &mut in_loop,
                loop_start: &mut loop_start,
                loop_length: &mut loop_length,
                endpoints: &endpoints
            };
            current_nodes.push(p);
        }
    }

    let mut total = 0;
    let mut end = false;
    while !end {
        end = true;


        /*
        let mut next_nodes = Vec::new();
        for &n in current_nodes.iter() {
            //print!("{n}");
            next_nodes.push(&nodes.get(n).unwrap()[directions[total % directions.len()] as usize]);
            end &= next_nodes.last().unwrap().ends_with('Z');
            //println!(" -> {}", next_nodes.last().unwrap());
        }
        current_nodes = next_nodes;
        total += 1;
        //println!("{total}\n");
        */
    }
    return total;
}

// code intended for LF .txt input
fn main() {
    let source = fs::read(&env::args().collect::<Vec<String>>()[1]).expect("failed to read file");

    //println!("{}", pt_1(&source));
    println!("{}", pt_2(&source));
}
