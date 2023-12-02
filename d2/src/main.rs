use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    let mut p1_answer: u32 = 0;

    'line_loop: for game_line in input.lines() {
        let mut parts = game_line.split(":");
        let (gameid_str, all_sets_str) = (parts.next().unwrap(), parts.next().unwrap());
        let gameid: u32 = gameid_str.split(" ").nth(1).unwrap().parse().unwrap();
        let sets_str = all_sets_str.split(";");

        for str in sets_str {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for w in str.split(",") {
                let mut parts = w.trim().split(" ");
                let (count, color) = (parts.next().unwrap().parse::<u32>().unwrap(), parts.next().unwrap());
                match color {
                    "red" => {
                        r += count;
                    }
                    "green" => {
                        g += count;
                    }
                    "blue" => {
                        b += count;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }

            if  r > max_r ||
                g > max_g || 
                b > max_b {
                continue 'line_loop;
            }
        }

        p1_answer += gameid;
    }

    println!("p1 answer: {p1_answer}");
}
