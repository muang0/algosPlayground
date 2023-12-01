use std::{error::Error, fs};

fn main() {
    let calibration_document_res = fs::read_to_string("./data/input.txt");
    let mut sum: u32 = 0;
    if let Ok(calibration_document) = calibration_document_res {
        for line in calibration_document.split("\n").into_iter() {
            let first_number = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let second_number = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let mut calibration_line: String = "".to_string();
            calibration_line.push(first_number);
            calibration_line.push(second_number);
            sum += calibration_line.parse::<u32>().unwrap();
        }
        println!("Sum: {:#}", sum);
    }
    
}
