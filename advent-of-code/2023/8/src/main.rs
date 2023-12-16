use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let mut directions = "";
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut num_steps = 0;
    let mut current_location = "AAA";
    for (line_number, line) in input.split("\n").enumerate() {
        if line_number == 0 {
            directions = line;
        } else if line_number != 1 {
            let location = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            map.insert(location, (left, right));
        }
    }
    'outer: loop {
        for direction in directions.chars().into_iter() {
            if direction == 'R' {
                current_location = map.get(current_location).unwrap().1;
            } else {
                current_location = map.get(current_location).unwrap().0;
            }
            num_steps += 1;
            if current_location == "ZZZ" {
                break 'outer
            }
        }
    }
    println!("number of steps: {:#}", num_steps);
}
