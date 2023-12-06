use std::{env::args, fs};

fn calculated_n_of_ways(time: &u64, dist: &u64) -> u64 {
    //fx = (t - x) * x - d 
    //fx = -x^2 + t - d

    let a = -1f64;
    let b = *time as f64;
    let c = -(*dist as f64);

    let delta = (b * b - 4f64 * a * c).sqrt();
    let x1 = (-b + delta) / (2f64 * a);
    let x2 = (-b - delta) / (2f64 * a);

    let mut ways = (x1 as u64 + 1).abs_diff(x2 as u64);
    if (x1 as u64 as f64) < x1 {
        ways += 1;
    }

    return ways;
}
fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut lines = input.lines();

    let times: Vec<u64> = lines.next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|str|str.parse().unwrap())
        .collect();

    let dists: Vec<u64> = lines.next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|str|str.parse().unwrap())
        .collect();

    let races = times.iter().zip(dists.iter());

    //p1 solution
    let mut p1_answer = 1;
    for (time, dist) in races {
        p1_answer *= calculated_n_of_ways(time, dist);
    }

    let p2_time_str: String = times.iter().map(|n|n.to_string()).collect();
    let p2_dist_str: String = dists.iter().map(|n|n.to_string()).collect();
    let p2_time: u64 = p2_time_str.parse().unwrap();
    let p2_dist: u64 = p2_dist_str.parse().unwrap();

    let p2_answer = calculated_n_of_ways(&p2_time, &p2_dist);

    println!("p1 answer: {p1_answer}");
    println!("p2 answer: {p2_answer}");
}
