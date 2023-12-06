use std::{env::args, fs, u32};

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut lines = input.lines();

    let times = lines.next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|str|str.parse::<f32>().unwrap());

    let dists = lines.next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|str|str.parse::<f32>().unwrap());

    let races = times.zip(dists);
    let mut p1_answer = 1;

    for (time, dist) in races {

        //fx = (t - x) * x - d 
        //fx = -x^2 + t - d
        
        let a = -1f32;
        let b = time;
        let c = -dist;

        let delta = (b * b - 4f32 * a * c).sqrt();
        let x1 = (-b + delta) / (2f32 * a);
        let x2 = (-b - delta) / (2f32 * a);

        let mut ways = (x1 as u32 + 1).abs_diff(x2 as u32);
        if (x1 as u32 as f32) < x1 {
            ways += 1;
        }

        p1_answer *= ways;
    }

    println!("p1 answer: {p1_answer}");
}
