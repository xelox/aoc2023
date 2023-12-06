use std::{env::args, fs, time::Instant};

fn calculated_n_of_ways(time: &u64, dist: &u64) -> u64 {
    //solving the quadratic equation:
    //fx = (t - x) * x - d 
    //fx = -x^2 + t - d

    let a = -1f64;
    let b = *time as f64;
    let c = -(*dist as f64);

    let delta = (b * b - 4f64 * a * c).sqrt();
    let x1 = (-b + delta) / (2f64 * a);
    let x2 = (-b - delta) / (2f64 * a);

    //calculating number of ways
    let mut ways = (x1 as u64 + 1).abs_diff(x2 as u64);
    //adjusting counted ways if x1 (and x2) are integers
    if (x1 as u64 as f64) < x1 {
        ways += 1;
    }

    return ways;
}

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    //parsing the input
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

    //constructing the parsed input into items of (time, dist)
    let races = times.iter().zip(dists.iter());

    //for benchmarking
    let mut start_t = Instant::now();

    //p1 solution
    let mut p1_answer = 1;
    for (time, dist) in races {
        p1_answer *= calculated_n_of_ways(time, dist);
    }
    let p1_bench = Instant::now() - start_t; //benchmarking

    //p2 solution
    //converting p1 times and distances to strings and joining
    let p2_time_str: String = times.iter().map(|n|n.to_string()).collect();
    let p2_dist_str: String = dists.iter().map(|n|n.to_string()).collect();
    //converting to actual numbers
    let p2_time: u64 = p2_time_str.parse().unwrap();
    let p2_dist: u64 = p2_dist_str.parse().unwrap();

    //actually calculating the p2 answer
    start_t = Instant::now(); //benchmarking
    let p2_answer = calculated_n_of_ways(&p2_time, &p2_dist);
    let p2_bench = Instant::now() - start_t; //benchmarking

    println!("p1 answer: {p1_answer} in {:.2?}", p1_bench);
    println!("p2 answer: {p2_answer} in {:.2?}", p2_bench);
}
