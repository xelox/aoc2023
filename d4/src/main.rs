use std::{env::args, fs, u32};

fn main() {
    let path = args().nth(1).unwrap();

    let input = fs::read_to_string(path)
        .unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut p1_answer: u32 = 0;

    let cards_count = lines.len();
    let mut scratch_cards = vec![1; cards_count];
     
    for (i, line) in lines.iter().enumerate() {
        let numbers_part = line.split(": ").nth(1).unwrap();
        let mut vert_split = numbers_part.split(" | ");

        let winning_nums: Vec<usize> = vert_split.next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.trim().parse().unwrap() )
            .collect();

        let card_nums: Vec<usize> = vert_split.next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.trim().parse().unwrap() )
            .collect();

        let mut wins_count: usize = 0;

        for num in card_nums.iter() {
            if winning_nums.contains(&num) {
                wins_count += 1;
            }
        }

        if wins_count > 0 {
            p1_answer += 1 * 2u32.pow(wins_count as u32 - 1);
        }

        let current_card_instances = scratch_cards[i];
        for w in 0..wins_count {
            scratch_cards[i + w + 1] += current_card_instances;
        }
    }

    let p2_answer = scratch_cards.iter().fold(0usize, |a, sc| a + sc);

    println!("p1 answer: {p1_answer}");
    println!("p2 answer: {p2_answer}");
}
