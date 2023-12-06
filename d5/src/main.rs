use std::{env::args, fs, u64, time::Instant};

struct AlmanacMap {
    dest_start: u64,
    src_start: u64,
    src_end: u64,
    range: u64,
}

impl AlmanacMap {
    fn from(list: Vec<u64>) -> AlmanacMap {
        AlmanacMap { 
            dest_start: list[0], 
            src_start: list[1], 
            src_end: list[1] + list[2],
            range: list[2]
        }
    }
}

#[derive(Clone, Copy)]
struct RangeDef {
    start: u64,
    count: u64,
}

enum IntersectionResult {
    Partial((RangeDef, RangeDef)),
    Full,
    None
}

/** tests one input range against another (cutter)
if there is a partial overlap will return two new ranges
the new ranges being the first and the second part of the original range
the cut being made at the point where the ranges intersect on the input range
the length of the input range is preseved in the sum of the output ranges
returns none if no cuts are to be made
*/
fn intersect_ranges(input: &RangeDef, cutter: &RangeDef) -> IntersectionResult {
    let input_end = input.start + input.count;
    let cutter_end = cutter.start + cutter.count - 1;

    //input inside the cutter range
    if input.start >= cutter.start && input_end <= cutter_end {
        return IntersectionResult::Full;
    }

    //range partially overlaps cutter
    if input.start <= cutter_end && input_end >= cutter.start {

        let a_start: u64 = input.start;
        let b_start: u64;
        if cutter.start < a_start {
            b_start = cutter_end + 1;
        } else {
            b_start = cutter.start;
        }
        let a_count = b_start - a_start - 1;
        let b_count = input.count - a_count - 1;

        let a = RangeDef{start: a_start, count: a_count};
        let b = RangeDef{start: b_start, count: b_count};

        return IntersectionResult::Partial((a, b));
    }

    //No overlap
    return IntersectionResult::None;
}

fn main() { 
    // env::set_var("RUST_BACKTRACE", "1");
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();

    let seeds: Vec<u64> = lines.next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n_str| n_str.parse().unwrap())
        .collect();

    let mut maps_sets: Vec<Vec<AlmanacMap>> = Vec::new(); 
    
    for line in lines {
        if line.is_empty() {
            maps_sets.push(Vec::new());
            continue;
        }
        if line.ends_with(":") {
            continue;
        }

        let almanac_map = AlmanacMap::from(line.split_whitespace()
            .map(|n_str| n_str.parse().unwrap())
            .collect()
        );

        maps_sets.last_mut()
            .unwrap()
            .push(almanac_map);
    }

    // min location number p1
    let mut p1_answer: u64 = u64::MAX;
    let start_time = Instant::now();

    //p1 solution
    //for each seed
    for seed in seeds.as_slice() {
        //we walk the map to map sequence
        let mut current_cue = seed.clone();
        for map_set in maps_sets.as_slice() {
            for map in map_set {
                let falls_into_range = current_cue >= map.src_start && current_cue < map.src_end;
                if falls_into_range {
                    let diff = current_cue - map.src_start;
                    current_cue = map.dest_start + diff; 
                    break;
                }
            }
        }
        if current_cue < p1_answer {
            p1_answer = current_cue;
        }
    } 

    let p1_elapsed = Instant::now() - start_time;

    

    //min location number p2
    let mut p2_answer: u64 = u64::MAX;

    //p2 solution
    //for each seed range
    for seeds_sr in seeds.chunks(2) {
        let mut ranges: Vec<RangeDef> = Vec::new();
        ranges.push(RangeDef{start: seeds_sr[0], count: seeds_sr[1]});

        let mut next_gen_ranges_to_shift: Vec<RangeDef> = Vec::with_capacity(100);
        let mut shifted_next_gen_ranges: Vec<RangeDef> = Vec::with_capacity(100);

        for map_set in maps_sets.as_slice() {
            for input_range in ranges.as_slice() {

                let mut found_intersection = false;
                for map in map_set {
                    let cutter = RangeDef{start: map.src_start, count: map.range};
                    match intersect_ranges(&input_range, &cutter) {
                        IntersectionResult::Partial((first, second)) => {
                            next_gen_ranges_to_shift.extend([first, second]);
                            found_intersection = true;
                            break;
                        } 
                        IntersectionResult::Full => {
                            let next_gen = RangeDef{
                                start: map.dest_start + input_range.start - map.src_start, 
                                count: input_range.count,
                            };
                            shifted_next_gen_ranges.push(next_gen);
                            found_intersection = true;
                            break;
                        }
                        IntersectionResult::None => {}
                    }
                }
                if !found_intersection {
                    next_gen_ranges_to_shift.push(*input_range);
                }
            }

            for ng in next_gen_ranges_to_shift.iter_mut() {
                for map in map_set {
                    if ng.start >= map.src_start && ng.start < map.src_end {
                        ng.start = map.dest_start + ng.start - map.src_start;
                        break;
                    }
                }
            }
            next_gen_ranges_to_shift.append(&mut shifted_next_gen_ranges);

            std::mem::swap(&mut ranges, &mut next_gen_ranges_to_shift);
            next_gen_ranges_to_shift.clear();
        }

        for current_ranges in ranges.as_slice() {
            if current_ranges.start < p2_answer {
                p2_answer = current_ranges.start;
            } 
        }
    }

    let p2_elapsed = Instant::now() - start_time;

    println!("p1 answer: {p1_answer} in {:.2?}", p1_elapsed);
    println!("p2 answer: {p2_answer} in {:.2?}", p2_elapsed);
}
