use std::{env::args, fs, collections::HashMap, u32};

// Searches around the strip number. returns some vector of gears coordinates.
// If no gears are found but other parts symbols are, it will return some empty vector.
// It will return none if no symbols are found at all.
fn search_gears_and_parts(matrix: &Vec<Vec<char>>, start: usize, end: usize, y: usize) -> Option<Vec<(usize, usize)>> {
    let mut gears_coords: Vec<(usize, usize)> = Vec::new();
    let mut has_addiacent_symbol = false;

    let mut left = start;
    if left > 0 {
        left -= 1;
        let c = matrix[y][left];
        if c != '.' {
            has_addiacent_symbol = true;
            if c == '*' {
                gears_coords.push((left, y));
            }
        }
    }

    let mut right = end;
    if right < matrix[y].len() - 1 {
        right += 1;
        let c = matrix[y][right];
        if c != '.' {
            has_addiacent_symbol = true;
            if c == '*' {
                gears_coords.push((right, y));
            }
        }
    }

    //nesting go vrroom vrrooom.
    if y > 0 || y < matrix.len() - 1 {
        for x in left..=right {
            if y > 0 {
                let c = matrix[y - 1][x];
                if c != '.' && !c.is_digit(10) {
                    has_addiacent_symbol = true;
                    if c == '*' {
                        gears_coords.push((x, y - 1));
                    }
                }
            }

            if y < matrix.len() - 1 {
                let c = matrix[y + 1][x];
                if c != '.' && !c.is_digit(10) {
                    has_addiacent_symbol = true;
                    if c == '*' {
                        gears_coords.push((x, y + 1));
                    }
                }
            }
        }
    }


    if has_addiacent_symbol {
        return Some(gears_coords);
    }
    return None;
}

fn main() {
    let path = args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let matrix: Vec<Vec<char>> = input.lines().map(|line|{
        line.chars().collect()
    }).collect();

    let mut p1_answer: u32 = 0;
    let mut p2_answer: u32 = 0;

    let mut gears_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    //going line by line
    for y in 0..matrix.len() {
        let line = &matrix[y];
        let mut strip_start: usize = 0;
        let mut number_str = String::from("");

        //defining a closure to avoid code duplication.
        let mut proccess_search_result = |search: Option<Vec<(usize, usize)>>, part_number: u32|{
            if search.is_some() {
                p1_answer += part_number;

                //for part 2...
                let gears_found = search.unwrap();
                for gear_coords in gears_found {
                    let connected_parts = gears_map.get_mut(&gear_coords);
                    match connected_parts {
                        Some(parts_list) => {
                            parts_list.push(part_number);
                        }
                        None => {
                            gears_map.insert(gear_coords, vec![part_number]);
                        }
                    }
                }
            }
        };

        //finding number strips
        for x in 0..line.len() {
            let c = &line[x];
            if c.is_digit(10) {
                if number_str.is_empty() {
                    strip_start = x;
                }
                number_str += &c.to_string();
            } else if !number_str.is_empty() {
                //searching symbols and prossessing the result.
                let search = search_gears_and_parts(&matrix, strip_start, x - 1, y);
                let part_number: u32 = number_str.to_string().parse().unwrap();
                proccess_search_result(search, part_number);
                number_str.clear();
            }
        }

        //checking for edge case where the line ended with a number
        if !number_str.is_empty() {
            //searching symbols and prossessing the result.
            let search = search_gears_and_parts(&matrix, strip_start, line.len() - 1, y);
            let part_number: u32 = number_str.to_string().parse().unwrap();
            proccess_search_result(search, part_number);
        }
    }

    //p2 proccessing each gear in gears_map.
    for gear_parts_list in gears_map.values() {
        //p2 only cares about gears connected to exactly 2 engine parts.
        if gear_parts_list.len() == 2 {
            let gear_ratio = gear_parts_list[0] * gear_parts_list[1]; 
            p2_answer += gear_ratio;
        }
    }

    println!("p1 answer: {p1_answer}");
    println!("p2 answer: {p2_answer}");
}
