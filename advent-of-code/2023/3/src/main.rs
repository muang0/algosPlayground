use std::collections::HashMap;
use std::fs;

struct Part {
    number: String,
    location: Vec<[u32; 2]>,
}

impl Part {
    fn new() -> Part {
        return Part {
            number: "".to_string(),
            location: Vec::new(),
        };
    }
}

fn main() {
    let mut gear_sum: u32 = 0;
    let mut schematic_symbols: HashMap<u32, HashMap<u32, bool>> = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    let engine_schematic = fs::read_to_string("./data/input.txt").unwrap();
    // populate vec<part> and schematicSymbols
    for (y_index, line) in engine_schematic.split("\n").into_iter().enumerate() {
        let mut part = Part::new();
        let mut line_schematic_symbols: HashMap<u32, bool> = HashMap::new();
        for (x_index, c) in line.chars().enumerate() {
            if c == '.' {
                // if full part has been read, push to parts vec
                if !part.number.is_empty() {
                    parts.push(part);
                    part = Part::new();
                }
            } else if c.is_numeric() {
                part.number.push(c);
                part.location
                    .push([y_index.try_into().unwrap(), x_index.try_into().unwrap()]);
            } else if c == '*' {
                // if full part has been read, push to parts vec
                if !part.number.is_empty() {
                    parts.push(part);
                    part = Part::new();
                }
                line_schematic_symbols.insert(x_index.try_into().unwrap(), true);
            } else {
                // if full part has been read, push to parts vec
                if !part.number.is_empty() {
                    parts.push(part);
                    part = Part::new();
                }
            }
        }
        // if full part has been read, push to parts vec
        if !part.number.is_empty() {
            parts.push(part);
        }
        if !line_schematic_symbols.is_empty() {
            schematic_symbols.insert(y_index.try_into().unwrap(), line_schematic_symbols);
        }
    }
    // for each part, determine if it should be included in the sum by searching for adjacent schematicSymbols
    let mut gears: HashMap<u32, HashMap<u32, Vec<String>>> = HashMap::new();
    // initalize gears hashmap
    for (y_index, _) in engine_schematic.split("\n").into_iter().enumerate() {
        gears.insert(y_index.try_into().unwrap(), HashMap::new());
    }
    // for each gear, find all adjacent parts
    for (y_index_gear, x_index_map) in schematic_symbols {
        for (x_index_gear, _) in x_index_map {
            let mut adjacent_gears: Vec<String> = Vec::new();
            for part in parts.iter() {
                for [y_index_part, x_index_part] in part.location.iter() {
                    // if part is within one unit of gear in any direction, add to gear vec
                    if (y_index_gear as i64 - *y_index_part as i64).abs() <= 1 && (x_index_gear as i64 - *x_index_part as i64).abs() <= 1 {
                        adjacent_gears.push(part.number.clone());
                        break
                    }
                }
            }
            // if only two parts adjacent to the gear, add to sum
            if adjacent_gears.len() == 2 {
                gear_sum += adjacent_gears.get(0).unwrap().parse::<u32>().unwrap() * adjacent_gears.get(1).unwrap().parse::<u32>().unwrap()
            }
        }
    }
    println!("gear sum: {:#}", gear_sum);
}
