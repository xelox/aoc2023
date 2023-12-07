use std::{env::args, fs, collections::HashMap, cmp::Ordering, u32};

#[derive(Eq, PartialEq)]
struct Hand{
    bid: u32,
    cards: [u8; 5],
    hand_type: u8,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type { return Some(Ordering::Greater); }
        if self.hand_type < other.hand_type { return Some(Ordering::Less); }
        self.cards.partial_cmp(&other.cards)
    }
}
    
impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        if self.hand_type > other.hand_type { return Ordering::Greater; }
        if self.hand_type < other.hand_type { return Ordering::Less; }
        self.cards.cmp(&other.cards)
    }
}

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut hands: Vec<Hand> = input.lines().map(|line|{
        let mut parts = line.split_ascii_whitespace();
        let cards_str: Vec<char> = parts.next().unwrap().chars().collect();
        let bid: u32 = parts.next().unwrap().parse().unwrap();
        let mut cards = [0; 5];
        let mut duplication_map: HashMap<u8, u8> = HashMap::new();
        for (i, card) in cards.iter_mut().enumerate() {
            let chr = cards_str[i];
            if chr.is_digit(10) {
                *card = chr as u8 - '0' as u8;
            } else {
                match chr {
                    'T' => { *card = 10; }                
                    'J' => { *card = 11; }                
                    'Q' => { *card = 12; }                
                    'K' => { *card = 13; }                
                    'A' => { *card = 14; }        
                    _ => { unreachable!(); }
                }
            }
            match duplication_map.get_mut(card) {
                Some(duplicate) => {
                    *duplicate += 1;
                }
                None => {
                    duplication_map.insert(*card, 1);
                }
            }
        }
        //duplication map values can be (5), (4, 1), (3, 2), (3, 1, 1), (2, 2, 1), (2, 1, 1, 1), (1, 1, 1, 1, 1)
        let hand_type: u8;
        match duplication_map.len() {
            1 => {hand_type = 6; }
            2 => {
                match duplication_map.values().max().unwrap() {
                    4 => { hand_type = 5; }
                    3 => { hand_type = 4; }
                    _ => { unreachable!(); }
                }
            }
            3 => {
                match duplication_map.values().max().unwrap() {
                    3 => { hand_type = 3; }
                    2 => { hand_type = 2; }
                    _ => { unreachable!(); }
                }
            }
            4 => { hand_type = 1; }
            5 => { hand_type = 0; }
            _ => { unreachable!(); }
        }

        return Hand{ hand_type, cards, bid};
    }).collect();

    hands.sort();


    let mut p2_answer: u32 = 0;
    for (rank, hand) in hands.iter().enumerate() {
        p2_answer += (rank as u32 + 1) * hand.bid;
    }
    println!("p2 answer: {p2_answer}");
}
