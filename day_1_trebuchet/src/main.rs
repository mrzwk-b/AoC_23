use std::fs;

fn completes_digit_name(letter: char, current_word: &mut String) -> Option<usize>{
    let mut new_word = String::new();
    if letter.is_ascii_digit() {
        *current_word = new_word;
        return Some(letter as usize - 48);
    }
    let digit_names = 
        ["zero", "one", "two", "three", "four", 
        "five", "six", "seven", "eight", "nine"];
    current_word.push(letter);
    let mut output: usize = 10;
    let mut done = false;
    let mut i = 0;
    while i < current_word.len() && !done {
        for digit in 0..10 {
            if digit_names[digit].starts_with(&current_word[i..]) {
                done = true;
                new_word = String::from(&current_word[i..]);
                if current_word == digit_names[digit] {
                    output = digit;
                }
            }
        }
        i += 1;
    }
    *current_word = new_word;
    if !done || output == 10 {return None;}
    else {return Some(output);}

}

fn main() {
    let source = fs::read_to_string("input.txt").expect("unable to read file");
    let mut stream = source.bytes();
    let mut current = stream.next();
    let mut sum = 0;
    while current != None {
        let mut current_char = current.expect("shouldn't this be logically impossible?") as char;
        let mut first = 10;
        let mut last = 10;
        let mut word = String::new();
        while current_char != '\n' {
            if let Some(x) = completes_digit_name(current_char, &mut word) {
                if first == 10 {
                    first = x;
                }
                last = x;
            }
            current_char = stream.next().expect("hit EOS inside inner loop") as char;
        }
        sum += (first as usize * 10) + last as usize;
        current = stream.next();
    }
    println!("{}", sum);
}
