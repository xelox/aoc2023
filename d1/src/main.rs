use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();
    let literal_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut p1_answer: u32 = 0;
    let mut p2_answer: u32 = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut p1_num = 0;

        let mut idx_s = line.len();
        let mut idx_e = 0;

        let mut p2_num_s: u8 = 0;
        let mut p2_num_e: u8 = 0;

        for i in 0..literal_digits.len() {
            let literal = literal_digits[i];
            if let Some(pos_num) = line.find(literal) {
                if pos_num < idx_s {
                    idx_s = pos_num;
                    p2_num_s = i as u8 + 1;
                }
            }
            if let Some(pos_num) = line.rfind(literal) {
                if pos_num > idx_e {
                    idx_e = pos_num;
                    p2_num_e = i as u8 + 1;
                }
            }
        }

        for i in 0..bytes.len() {
            let b = bytes[i];
            let u = b - '0' as u8;
            if u <= 9 { 
                p1_num = u * 10;
                if i < idx_s {
                    p2_num_s = u;
                }
                break;
            }
        }
        for i in (0..bytes.len()).rev() {
            let b = bytes[i];
            let u = b - '0' as u8;
            if u <= 9 {
                p1_num += u;
                if i >= idx_e {
                    p2_num_e = u;
                }
                break;
            }
        }

        p1_answer += p1_num as u32;
        p2_answer += (p2_num_s * 10 + p2_num_e) as u32;
    }
    println!("Part 1 answer: {p1_answer}");
    println!("Part 2 answer: {p2_answer}");
}
