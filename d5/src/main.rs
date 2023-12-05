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

fn intersect_ranges(range: &(u64, u64), cutter: &(u64, u64)) -> Option<Vec<(u64, u64)>> {
    let mut result: Vec<(u64, u64)> = Vec::new();

    let (range_start, range_size) = range;
    let range_end = range_start + range_size;

    let (cutter_start, cutter_size) = cutter;
    let cutter_end = cutter_start + cutter_size - 1;

    //range inside the cutter range
    if range_start >= cutter_start && range_end <= cutter_end {
        return None;
    }

    //range partially overlap cutter
    if range_start <= &cutter_end && range_end >= *cutter_start {

        let a_start: u64 = *range_start;
        let b_start: u64;
        if cutter_start < &a_start {
            b_start = cutter_end + 1;
        } else {
            b_start = *cutter_start;
        }
        let a_count = b_start - a_start - 1;
        let b_count = range_size - a_count - 1;

        result.push( (a_start, a_count) );
        result.push( (b_start, b_count) );

        return Some(result);
    }

    return None;
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
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        ranges.push((seeds_sr[0], seeds_sr[1]));
        let mut next_gen_ranges: Vec<(u64, u64)> = Vec::new();

        for map_set in maps_sets.as_slice() {
            for range in ranges.as_slice() {
                let mut found_intersection = false;
                for map in map_set {
                    let search_intersect = intersect_ranges(&range, &(map.src_start, map.range));
                    match search_intersect {
                        Some(new_ranges) => {
                            next_gen_ranges = [next_gen_ranges, new_ranges].concat();
                            found_intersection = true;
                            break;
                        } 
                        None => {}
                    }
                }
                if !found_intersection {
                    next_gen_ranges.push(*range);
                }
            }

            for ng in next_gen_ranges.iter_mut() {
                for map in map_set {
                    let falls_into_range = ng.0 >= map.src_start && ng.0 < map.src_end;
                    if falls_into_range {
                        let diff = ng.0 - map.src_start;
                        ng.0 = map.dest_start + diff; 
                        break;
                    }
                }
            }
            ranges = next_gen_ranges.clone();
            next_gen_ranges.clear();
        }

        for (range_start, _) in ranges.as_slice() {
            if range_start < &p2_answer {
                p2_answer = *range_start;
            } 
        }
    }

    let p2_elapsed = Instant::now() - start_time;

    println!("p1 answer: {p1_answer} in {:.2?}", p1_elapsed);
    println!("p2 answer: {p2_answer} in {:.2?}", p2_elapsed);
}
