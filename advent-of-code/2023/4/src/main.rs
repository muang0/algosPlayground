use std::fs::read_to_string;

fn main() {
    let cards = read_to_string("data/input.txt").unwrap();
    let mut total_points = 0;
    for card in cards.split("\n") {
        let mut card_points = 0;
        let winning_numbers: Vec<&str> = card.split("|").nth(0).unwrap().split(":").nth(1).unwrap().split(" ").filter(|&num| num != "").collect();
        let elfs_numbers: Vec<&str> = card.split("|").nth(1).unwrap().split(" ").filter(|&num| num != "").collect();
        for elf_number in elfs_numbers {
            if winning_numbers.contains(&elf_number) {
                card_points = match card_points {
                    0 => 1,
                    _ => card_points * 2
                }
            }
        }
        total_points += card_points;
    }
    println!("Total points: {:#}", total_points);
}
