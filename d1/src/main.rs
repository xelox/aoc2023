use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();
    let mut sum: u32 = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut num = 0;
        for c in bytes {
            let u = c - '0' as u8;
            if u <= 9 { 
                num = u * 10;
                break;
            }
        }
        for c in bytes.iter().rev() {
            let u = c - '0' as u8;
            if u <= 9 {
                num += u;
                break;
            }
        }
        sum += num as u32;
    }
    println!("Part 1 answer: {sum}");
}
