use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    let mut p1_answer: u32 = 0;
    let mut p2_answer: u32 = 0;

    //parsing lines (games)
    for game_line in input.lines() {
        let mut parts = game_line.split(":");
        let (gameid_str, all_sets_str) = (parts.next().unwrap(), parts.next().unwrap());
        let gameid: u32 = gameid_str.split(" ").nth(1).unwrap().parse().unwrap();

        let mut p1_is_valid_game: bool = true;

        let mut max_r_draw = 0;
        let mut max_g_draw = 0;
        let mut max_b_draw = 0;

        //for each game, parsin sets of cube draws.
        for str in all_sets_str.split(";") {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            //for each set, parsing individual cube draws.
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

            if r > max_r_draw {
                max_r_draw = r;
            }
            if g > max_g_draw {
                max_g_draw = g;
            }
            if b > max_b_draw {
                max_b_draw = b;
            }

            if  r > max_r ||
                g > max_g || 
                b > max_b {
                p1_is_valid_game = false;
            }
        }

        let power = max_r_draw * max_g_draw * max_b_draw;
        p2_answer += power;

        if p1_is_valid_game {
            p1_answer += gameid;
        }
    }

    println!("p1 answer: {p1_answer}");
    println!("p2 answer: {p2_answer}");
}
