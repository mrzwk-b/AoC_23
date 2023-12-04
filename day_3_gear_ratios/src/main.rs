use std::fs;

fn is_symbol (character: u8) -> bool {
    !(character == b'.' || (b'0'..b'9').contains(&character))
}

fn main() {
    // read input
    let source = fs::read("input.txt").expect("failed to read input file");
    let mut row_length = 0;
    while source[row_length] != b'\n' {
        row_length += 1;
    }
    row_length += 1;
    let bottom_row = source.len() - row_length;
    // iterate over it
    let mut in_digits = false;
    let mut hit_symbol = false;
    let mut digits_total = 0;
    let mut schema_total = 0;
    for i in 0..source.len() {
        if (48..58).contains(&source[i]) {
            // check for symbols only in places where the grid exists and hasn't been checked in this digit string yet
            let column = i % row_length;
            if !in_digits {
                hit_symbol |= 
                    (column != 0 && (
                        (i > row_length && is_symbol(source[i - row_length - 1])) ||
                        (i < bottom_row && is_symbol(source[i + row_length - 1])) ||
                        is_symbol(source[i - 1])
                    )) ||
                    (i > row_length && is_symbol(source[i - row_length])) ||
                    (i < bottom_row && is_symbol(source[i + row_length]))
                ;
            }
            if column != row_length - 2 {
                hit_symbol |=
                    (i > row_length && is_symbol(source[i - row_length + 1])) ||
                    (i < bottom_row && is_symbol(source[i + row_length + 1])) ||
                    is_symbol(source[i + 1])
                ;
            }
            digits_total = (digits_total * 10) + (source[i] as usize - 48);
            in_digits = true;
        }
        else if in_digits {
            schema_total += digits_total * (hit_symbol as usize);
            digits_total = 0;
            hit_symbol = false;
            in_digits = false;
        }
        
    }
    println!("{}", schema_total);
}
