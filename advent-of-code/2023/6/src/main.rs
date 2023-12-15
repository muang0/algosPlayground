fn main() {
    let file = std::fs::read_to_string("data/input.txt").unwrap();
    let times: Vec<u32> = file.lines().nth(0).unwrap().split(" ").filter(|x| x.chars().all(|c: char| c.is_numeric()) && *x != "").map(|x| x.parse().unwrap()).collect();
    let distances: Vec<u32> = file.lines().nth(1).unwrap().split(" ").filter(|x| x.chars().all(|c: char| c.is_numeric()) && *x != "").map(|x| x.parse().unwrap()).collect();
    let mut race_data: Vec<(u32, u32)> = Vec::new();
    for (i, _) in times.iter().enumerate() {
        race_data.push((*times.iter().nth(i).unwrap(), *distances.iter().nth(i).unwrap()))
    }
    let mut product = 1;
    for (time, distance) in race_data {
        // for 0..time, determine if respective distance would be greater than &distance, and calculate sum of occurrances
        let mut num_ways_to_win = 0;
        for t in 0..time {
            let distance_simulated = t*(time-t);
            if distance_simulated > distance {
                num_ways_to_win +=1;
            }
        }
        product *= num_ways_to_win;
    }
    println!("product: {:#}", product);
}
