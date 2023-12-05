use std::{fs, env};

struct FormattedSource<'a> {
    content: &'a Vec<u8>,
    row_length: usize,
    bottom_row: usize
}

fn is_symbol(character: u8) -> bool {
    !(character == b'.' || (48..58).contains(&character))
}

fn get_part_length(
    start: usize, 
    FormattedSource{
        content,
        row_length,
        bottom_row
    }: &FormattedSource
) -> Option<usize> 
{
    let mut length = 0;
    let mut hit_symbol = false;
    let mut i = start;
    while i < content.len() && (48..58).contains(&content[i]) {
        let column = i % row_length;
        if i == start {
            hit_symbol |= 
                (column != 0 && (
                    (i > *row_length && is_symbol(content[i - *row_length - 1])) ||
                    (i < *bottom_row && is_symbol(content[i + *row_length - 1])) ||
                    is_symbol(content[i - 1])
                )) ||
                (i > *row_length && is_symbol(content[i - *row_length])) ||
                (i < *bottom_row && is_symbol(content[i + *row_length]))
            ;
        }
        if column != row_length - 2 {
            hit_symbol |=
                (i > *row_length && is_symbol(content[i - *row_length + 1])) ||
                (i < *bottom_row && is_symbol(content[i + *row_length + 1])) ||
                is_symbol(content[i + 1])
            ;
        }
        i += 1;
        length += 1;

    }
    if !hit_symbol || length == 0 {
        return None;
    }
    return Some(length);
}

fn get_part_value(
    start: usize,
    length: usize,
    FormattedSource{content, ..}: &FormattedSource
) -> usize
{
    let mut digits_total = 0;
    for i in 0..length {
        digits_total = (digits_total * 10) + (content[start + i] as usize - 48);
    }
    return digits_total;
}

fn pt_1(source: FormattedSource) -> usize {  
    let mut parts_total = 0;
    let mut i = 0;
    while i < source.content.len() {
        if let Some(length) = get_part_length(i, &source) {
            parts_total += get_part_value(i, length, &source);
            i += length;
        } 
        else {
            i += 1;
        }
    }
    parts_total
}

// still incomplete
fn pt_2(FormattedSource{content, row_length, bottom_row}: FormattedSource) -> usize {
    let mut gears_total = 0;
    let mut i = 0;
    while i < content.len() {
        if content[i] == b'*' {
            let row_start =  i - (i % row_length);
            let mut parts: Vec<(usize, usize)> = Vec::new();
            let mut above_countdown = 0;
            let mut level_countdown = 0;
            let mut below_countdown = 0;
            let mut j = row_start;
            while j < row_length {
                //above
                if i > row_length && above_countdown == 0 {
                    if let Some(length) = get_part_length(row_start - row_length + j, &FormattedSource {content, row_length, bottom_row}) {
                        parts.push((row_start - row_length + j, length));
                        above_countdown = length - 1;
                    }
                }
                //level
                if level_countdown == 0 {
                    if let Some(length) =
                        get_part_length(row_start + j, &FormattedSource {content, row_length, bottom_row}) 
                    {
                        parts.push((row_start + j, length));
                        level_countdown = length - 1;
                    }
                }
                //below
                if i < bottom_row && below_countdown == 0 {
                    if let Some(length) =
                        get_part_length(row_start + row_length + j, &FormattedSource {content, row_length, bottom_row}) 
                    {
                        parts.push((row_start + row_length + j, length));
                        below_countdown = length - 1;
                    }
                }
                j += 1;
            }
            if parts.len() == 2 {
                gears_total += 
                    get_part_value(parts[0].0, parts[0].1, &FormattedSource{content, row_length, bottom_row}) *
                    get_part_value(parts[1].0, parts[1].1, &FormattedSource{content, row_length, bottom_row})
                ;
            }
        }
        i += 1;
    }
    gears_total
}

// code designed for CRLF .txt input
fn main() {
    let args: Vec<String> = env::args().collect();
    let content = &fs::read(&args[1]).expect("failed to read input file");
    let mut row_length = 0;
    while content[row_length] != b'\n' {
        row_length += 1;
    }
    row_length += 1;
    let bottom_row = content.len() - row_length;
    let source = FormattedSource{content, row_length, bottom_row};

    println!("{}", pt_2(source));
}
