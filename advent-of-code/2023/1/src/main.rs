use std::{error::Error, fs};
use regex::Regex;

fn main() {
    let calibration_document_res = fs::read_to_string("./data/input.txt");
    let mut sum_part_one: u32 = 0;
    let mut sum_part_two: u32 = 0;
    if let Ok(calibration_document) = calibration_document_res {
        for line in calibration_document.split("\n").into_iter() {
            // part one
            let first_number: char = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_number = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let mut calibration_line: String = "".to_string();
            calibration_line.push(first_number);
            calibration_line.push(last_number);
            sum_part_one += calibration_line.parse::<u32>().unwrap();
            // part two
            let re = Regex::new(r"([1-9]|(twone)|(oneight)|(nineight)|(eightwo)|(eighthree)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))").unwrap();
            let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
            let first_number_str = *matches.first().unwrap();
            let last_number_str = *matches.last().unwrap();
            let first_number: char = match first_number_str {
                "twone" => '2',
                "oneight" => '1',
                "nineight" => '9',
                "eightwo" => '8',
                "eighthree" => '8',
                "one" => '1',
                "two" => '2',
                "three" => '3',
                "four" => '4',
                "five" => '5',
                "six" => '6',
                "seven" => '7',
                "eight" => '8',
                "nine" => '9',
                _ => first_number_str.chars().into_iter().collect::<Vec<char>>()[0]
            };
            let last_number: char = match last_number_str {
                "twone" => '1',
                "oneight" => '8',
                "nineight" => '8',
                "eightwo" => '2',
                "eighthree" => '3',
                "one" => '1',
                "two" => '2',
                "three" => '3',
                "four" => '4',
                "five" => '5',
                "six" => '6',
                "seven" => '7',
                "eight" => '8',
                "nine" => '9',
                _ => last_number_str.chars().into_iter().collect::<Vec<char>>()[0]
            };
            let mut calibration_line: String = "".to_string();
            calibration_line.push(first_number);
            calibration_line.push(last_number);
            sum_part_two += calibration_line.parse::<u32>().unwrap();

            // println!("{:#} {:#}", calibration_line, line);
        }
        println!("Sum part one: {:#}", sum_part_one);
        println!("Sum part two: {:#}", sum_part_two);
    }
    
}
