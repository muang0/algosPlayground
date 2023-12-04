use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let cards = read_to_string("data/input.txt").unwrap();
    let mut total_cards: u32 = 0;
    let mut card_copies: HashMap<u32, u32> = HashMap::new();
    // initialize card_copies hashmap
    for card_num in 1..=cards.split("\n").count() {
        card_copies.insert(card_num.try_into().unwrap(), 1);
    }
    for (card_num, card) in cards.split("\n").enumerate() {
        let mut wining_num_count = 0;
        let winning_numbers: Vec<&str> = card
            .split("|")
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|&num| num != "")
            .collect();
        let elfs_numbers: Vec<&str> = card
            .split("|")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|&num| num != "")
            .collect();
        for elf_number in elfs_numbers {
            if winning_numbers.contains(&elf_number) {
                wining_num_count += 1
            }
        }
        // for each card copy, increment total_cards, and subsequent won copies
        let current_card_num = card_num as u32 + 1;
        for _ in 1..=*card_copies.get(&current_card_num).unwrap() {
            total_cards += 1;
            for win_count in 1..wining_num_count+1 {
                let card_to_increment = card_copies.get(&(current_card_num + win_count));
                if let Some(card_to_increment) = card_to_increment {
                    card_copies.insert(current_card_num + win_count, card_to_increment + 1);
                }
            }
        }
    }
    println!("Total cards: {:#}", total_cards);
}
