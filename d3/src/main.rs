use std::{env::args, fs};

fn has_addiacent_symbol(matrix: &Vec<Vec<char>>, start: usize, end: usize, y: usize) -> bool {
    let mut left = start;
    if left > 0 {
        left -= 1;
        let c = matrix[y][left];
        if c != '.' {
            return true;
        }
    }
    let mut right = end;
    if right < matrix[y].len() - 1 {
        right += 1;
        let c = matrix[y][right];
        if c != '.' {
            return true;
        }
    }
    if y > 0 {
        for x in left..=right {
            let c = matrix[y - 1][x];
            if c != '.' && !c.is_digit(10) {
                return true;
            }
        }
    }
    if y < matrix.len() - 1 {
        for x in left..=right {
            let c = matrix[y + 1][x];
            if c != '.' && !c.is_digit(10) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let matrix: Vec<Vec<char>> = input.lines().map(|line|{
        line.chars().collect()
    }).collect();

    let mut p1_answert: u32 = 0;

    //going line by line
    for y in 0..matrix.len() {
        let line = &matrix[y];
        let mut strip_start: usize = 0;
        let mut number_str = String::from("");

        //finding number strips
        for x in 0..line.len() {
            let c = &line[x];
            if c.is_digit(10) {
                if number_str.is_empty() {
                    strip_start = x;
                }
                number_str += &c.to_string();
            } else if !number_str.is_empty() {
                // if valid engine part number, add the value to p1_answert
                if has_addiacent_symbol(&matrix, strip_start, x - 1, y) {
                    p1_answert += number_str.parse::<u32>().unwrap();
                }
                number_str.clear();
            }
        }

        //checking for edge case where the line ended with a number
        if !number_str.is_empty() {
            if has_addiacent_symbol(&matrix, strip_start, line.len() - 1, y) {
                p1_answert += number_str.parse::<u32>().unwrap();
            }
        }
    }

    println!("p1 answer: {p1_answert}");
}
