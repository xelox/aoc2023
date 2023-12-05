use std::{env::args, fs, u32};

struct AlmanacMap {
    dest: u32,
    src: u32,
    range: u32
}

impl AlmanacMap {
    fn from(list: Vec<u32>) -> AlmanacMap {
        AlmanacMap { 
            dest: list[0], 
            src: list[1], 
            range: list[2] 
        }
    }
}

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();

    let seeds: Vec<u32> = lines.next()
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

    // min location number
    let mut p1_answer: u32 = u32::MAX;

    //for each seed
    for seed in seeds {
        //we walk the map to map sequence
        let mut current_cue = seed;
        for map_set in maps_sets.as_slice() {
            for map in map_set {
                let falls_into_range = current_cue >= map.src && current_cue < map.src + map.range;
                if falls_into_range {
                    let diff = current_cue - map.src;
                    current_cue = map.dest + diff; 
                    break;
                }
            }
        }
        if current_cue < p1_answer {
            p1_answer = current_cue;
        }
    } 

    println!("p1 answer: {p1_answer}");
}
