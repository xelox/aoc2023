use std::{env::args, fs};

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut p1_answer: u32 = 0;
    
    for line in input.lines() {
        let numbers_part = line.split(": ").nth(1).unwrap();
        let mut vert_split = numbers_part.split(" | ");

        let winning_nums: Vec<u32> = vert_split.next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.trim().parse::<u32>().unwrap() )
            .collect();

        let card_nums = vert_split.next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.trim().parse::<u32>().unwrap() );

        let mut wins_count = 0;

        for num in card_nums {
            if winning_nums.contains(&num) {
                wins_count += 1;
            }
        }

        if wins_count > 0 {
            p1_answer += 1 * (2 as u32).pow(wins_count - 1);
        }
    }

    println!("p1 answer: {p1_answer}");
}
