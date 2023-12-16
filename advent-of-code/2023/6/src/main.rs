fn main() {
    let file = std::fs::read_to_string("data/input.txt").unwrap();
    let time: String = file.lines().nth(0).unwrap().replace(" ", "");
    let time: u64 = time.as_str().split(":").nth(1).unwrap().parse().unwrap();
    let distance: String = file.lines().nth(1).unwrap().replace(" ", "");
    let distance: u64 = distance.as_str().split(":").nth(1).unwrap().parse().unwrap();
    let mut lower_bound = 0;
    for t in 0..time {
        let distance_simulated = t*(time-t);
        if distance_simulated > distance {
            lower_bound = t;
            break;
        }
    }
    let num_ways_to_win = time-(lower_bound*2-1);
    println!("product: {:#}", num_ways_to_win);
}
