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

// schematicSymbols: hashmap<hashmap><bool>

fn main() {
    let mut parts_sum: u32 = 0;
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
            } else {
                // if full part has been read, push to parts vec
                if !part.number.is_empty() {
                    parts.push(part);
                    part = Part::new();
                }
                line_schematic_symbols.insert(x_index.try_into().unwrap(), true);
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
    for part in parts {
        let mut is_next_to_symbol = false;
        'outer: for index in part.location {
            for [y_index, x_index] in [[0, 1], [1, 0], [1, 1]] {
                if let Some(line_map) = schematic_symbols.get(&index[0].wrapping_add(y_index)) {
                    if line_map.get(&index[1].wrapping_add(x_index)) == Some(&true) {
                        is_next_to_symbol = true;
                        break 'outer;
                    }
                }
                if let Some(line_map) = schematic_symbols.get(&index[0].wrapping_sub(y_index)) {
                    if line_map.get(&index[1].wrapping_sub(x_index)) == Some(&true) {
                        is_next_to_symbol = true;
                        break 'outer;
                    }
                }
                if let Some(line_map) = schematic_symbols.get(&index[0].wrapping_add(y_index)) {
                    if line_map.get(&index[1].wrapping_sub(x_index)) == Some(&true) {
                        is_next_to_symbol = true;
                        break 'outer;
                    }
                }
                if let Some(line_map) = schematic_symbols.get(&index[0].wrapping_sub(y_index)) {
                    if line_map.get(&index[1].wrapping_add(x_index)) == Some(&true) {
                        is_next_to_symbol = true;
                        break 'outer;
                    }
                }
            }
        }
        if is_next_to_symbol {
            parts_sum += part.number.parse::<u32>().unwrap();
        }
    }
    println!("Parts sum: {:#}", parts_sum);
}
