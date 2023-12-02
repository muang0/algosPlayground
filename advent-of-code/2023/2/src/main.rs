use std::fs;
use std::collections::HashMap;

fn main() {
    let mut game_sum: i32 = 0;
    let game_doc = fs::read_to_string("./data/input.txt").unwrap();
    for line in game_doc.split("\n") {
        let mut min_cubes = HashMap::new();
        min_cubes.insert("red", 0);
        min_cubes.insert("blue", 0);
        min_cubes.insert("green", 0);
        for round in line.split(":").nth(1).unwrap().split(";") {
            for cube_set in round.split(",") {
                let num_cubes = cube_set.split(" ").nth(1).unwrap().to_string().parse::<i32>().unwrap();
                let cubes_color = cube_set.split(" ").nth(2).unwrap();
                if *min_cubes.get(cubes_color).unwrap() < num_cubes {
                    min_cubes.insert(cubes_color, num_cubes);
                }
            }
        }
        let round_power = min_cubes.get("red").unwrap() * min_cubes.get("green").unwrap() * min_cubes.get("blue").unwrap();
        game_sum += round_power;
    }
    println!("Game sum: {:#}", game_sum);
}
