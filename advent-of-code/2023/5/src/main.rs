
fn main() {
    let mut seeds: Vec<&str> = Vec::new();
    let mut closest_distance = u64::MAX;
    let almanac_input = std::fs::read_to_string("data/input.txt").unwrap();
    let seeds: Vec<&str> = almanac_input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|i| *i != "")
        .collect();
    for seed in seeds {
        let mut almanac_tracker: u64 = seed.parse().unwrap();
        let mut value_in_almanac_section = false;
        for (line_number, line) in almanac_input.split("\n").enumerate() {
            if line_number == 0 {
                continue;
            } else if line.contains(":") {
                value_in_almanac_section = false;
            } else if line != "" {
                let destination_range_start: u64 = line.split(" ").nth(0).unwrap().parse().unwrap();
                let source_range_start: u64 = line.split(" ").nth(1).unwrap().parse().unwrap();
                let range_length: u64 = line.split(" ").nth(2).unwrap().parse().unwrap();
                // if almanac tracker is present in the source mapping, convert the value for the section
                if !value_in_almanac_section {
                    if almanac_tracker >= source_range_start && almanac_tracker < source_range_start + range_length {
                        let increment = almanac_tracker - source_range_start;
                        almanac_tracker = destination_range_start + increment;
                        value_in_almanac_section = true;
                    }
                }
            }
        }
        if almanac_tracker < closest_distance {
            closest_distance = almanac_tracker;
        }
    }
    println!("Closest seed: {:#}", closest_distance);
}
