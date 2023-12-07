use std::{env::args, fs, collections::HashMap, cmp::Ordering, u32, u8};

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

/** 
* duplication map values can only be one of 
* (5), (4, 1), (3, 2), (3, 1, 1), (2, 2, 1), (2, 1, 1, 1), (1, 1, 1, 1, 1)
* so by looking at the lenght of the map and (when needed) the max duplicate cards...
* it's possible to determine the type of hand...
* Five of a Kind, Four of a Kind, Full House, Two Pair, One Pair, High Card
*/
fn compute_hand_type(duplication_map: &HashMap<u8, u8>) -> u8 {
    match duplication_map.len() {
        1 => {return 6; }
        2 => {
            match duplication_map.values().max().unwrap() {
                4 => { return 5; }
                3 => { return 4; }
                _ => { unreachable!(); }
            }
        }
        3 => {
            match duplication_map.values().max().unwrap() {
                3 => { return 3; }
                2 => { return 2; }
                _ => { unreachable!(); }
            }
        }
        4 => { return 1; }
        5 => { return 0; }
        _ => { unreachable!(); }
    }
}

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    // mapping every line of the input to two vectors of Hand structs 
    // formatted for p1 and p2 respectively
    let (mut p1_hands, mut p2_hands): (Vec<Hand>, Vec<Hand>) = input.lines().map(|line|{
        // first for each line, extract the bid amound and the hand string representation
        let mut parts = line.split_ascii_whitespace();
        let cards_str: Vec<char> = parts.next().unwrap().chars().collect();
        let bid: u32 = parts.next().unwrap().parse().unwrap();

        // p1 and p2 cards sequences to be mapped
        let mut p1_cards = [0; 5];
        let mut p2_cards = [0; 5];

        // for tracking duplication counts
        let mut duplication_map: HashMap<u8, u8> = HashMap::new();
        let mut has_joker = false;

        // walking the cards sring and mapping the chars to card values
        for (i, (p1_card, p2_card)) in p1_cards.iter_mut().zip(p2_cards.iter_mut()).enumerate() {
            let chr = cards_str[i];

            if chr.is_digit(10) {
                *p1_card = chr as u8 - '0' as u8;
                *p2_card = *p1_card;
            } else {
                match chr {
                    'T' => { *p1_card = 10; *p2_card = 10 }
                    'J' => { *p1_card = 11; *p2_card = 1; has_joker = true; } /*sike*/
                    'Q' => { *p1_card = 12; *p2_card = 12 }
                    'K' => { *p1_card = 13; *p2_card = 13 }
                    'A' => { *p1_card = 14; *p2_card = 14 }
                    _ => { unreachable!(); }
                }
            }

            // tracking duplication logic
            match duplication_map.get_mut(p1_card) {
                Some(duplicate) => {
                    *duplicate += 1;
                }
                None => {
                    duplication_map.insert(*p1_card, 1);
                }
            }
        }

        // categorizing p1 hand type.
        let p1_hand_type: u8 = compute_hand_type(&duplication_map);
        let p2_hand_type: u8;

        // in this case duplication_map needs to be addapted for p2
        if has_joker {
            let jokers_count = duplication_map.remove(&11).unwrap();
            if jokers_count != 5 { 
                // if the joker count was 5 the duplication_map would now be
                // empty and calling unwrap on the next chain would crash the program
                let (_, dups) = duplication_map.iter_mut().max_by_key(|i| i.1.clone()).unwrap();
                *dups += jokers_count;
                p2_hand_type = compute_hand_type(&duplication_map);
            }
            else { p2_hand_type = p1_hand_type; }
        } else { p2_hand_type = p1_hand_type; }

        // creating and returning the p1 and p2 Hand variants for this hand line
        return (
            Hand{ hand_type: p1_hand_type, cards: p1_cards, bid},
            Hand{ hand_type: p2_hand_type, cards: p2_cards, bid}
        );
    }).unzip();

    // by simply sorting the vectors, the index of each hand will also be it's rank (-1)
    p1_hands.sort();
    p2_hands.sort();


    //and finally just iterating all the hands to calculate the answer(s)
    let mut p1_answer: u32 = 0;
    let mut p2_answer: u32 = 0;
    for (i, (p1_hand, p2_hand)) in p1_hands.iter().zip(p2_hands.iter()).enumerate() {
        let rank = i as u32 + 1;
        p1_answer += rank * p1_hand.bid;
        p2_answer += rank * p2_hand.bid;
    }


    println!("p1 answer: {p1_answer}");
    println!("p2 answer: {p2_answer}");
}
