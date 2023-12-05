use std::{fs, env};

// returned tuple formatted as (destination start, source start, range lenght) just like in document
fn parse_line(input: &Vec<u8>, index: &mut usize) -> [usize; 3] {
    let mut line_values: [usize; 3] = [0, 0, 0];
    for i in 0..3 {
        let mut total = 0;
        while (48..58).contains(&input[*index]) {
            total = (total * 10) + (input[*index] as usize - 48);
            *index += 1;
        }
        line_values[i] = total;
        *index += 1;
    }
    return line_values;
}

fn remap(input: &Vec<u8>, start_data: &Vec<usize>, index: &mut usize) -> Vec<usize> {
    let mut end_data = start_data.clone();
    while input[*index] != b'\n' {
        let map = parse_line(&input, index);
        for j in 0..start_data.len() {
            if (map[1]..map[2]).contains(&start_data[j]) {
                end_data[j] = map[0] + (start_data[j] - map[1]);
            }
        }
    }
    return end_data;
}

fn pt_1(input: Vec<u8>) -> usize {
    let mut table: [Vec<usize>; 8];

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read(&args[1]).expect("failed to read input");
    
    println!("{}", pt_1(input));
}
