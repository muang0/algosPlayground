use std::fs;

fn main() {
    let mut game_sum: u32 = 0;
    let game_doc = fs::read_to_string("./data/input.txt").unwrap();
    for line in game_doc.split("\n") {
        let game_id = line.split(" ").nth(1).unwrap().split(":").nth(0).unwrap().to_string().parse::<u32>().unwrap();
        let mut game_is_possible = true;
        for round in line.split(":").nth(1).unwrap().split(";") {
            for cube_set in round.split(",") {
                let num_cubes = cube_set.split(" ").nth(1).unwrap().to_string().parse::<i32>().unwrap();
                let cubes_color = cube_set.split(" ").nth(2).unwrap();
                let max_cubes_allowed = match cubes_color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("unknown color encountered")
                };
                if num_cubes > max_cubes_allowed {
                    game_is_possible = false;
                    break;
                }
            }
        }
        if game_is_possible {
            game_sum += game_id;
        }
    }
    println!("Game sum: {:#}", game_sum);
}
