use std::{env::args, fs};

struct ScratchCards {
    instances: usize,
    wins_count: usize,
}

impl ScratchCards {
    fn new(wins_count: usize) -> ScratchCards {
        ScratchCards {
            instances: 1,
            wins_count
        }
    }
}

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut p1_answer: u32 = 0;
    let mut p2_answer: u32 = 0;

    let mut scratch_cards: Vec<ScratchCards> = Vec::new();
     
    for line in input.lines() {
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
            p1_answer += 1 * (2 as u32).pow(wins_count as u32 - 1);
        }

        scratch_cards.push(ScratchCards::new(wins_count));
    }

    loop {
        let mut operations_done = 0;

        for i in 0..scratch_cards.len() {
            let sc = &mut scratch_cards[i];

            if sc.instances == 0 {
                continue;
            }

            operations_done += 1;
            p2_answer += 1;

            sc.instances -= 1; 

            for w in 0..sc.wins_count {
                let next_card = &mut scratch_cards[i + w + 1];
                next_card.instances += 1;
            }
        }

        if operations_done == 0 {
            break;
        }
    }

    println!("p1 answer: {p1_answer}");
    println!("p2 answer: {p2_answer}");
}
