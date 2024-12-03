use std::{fs::File, io::{BufRead, BufReader}};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mults: Vec<i32> = Vec::new();

    let file = File::open("./days/day3/input.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    for line in reader.lines() {
        let line = line?;
        for mult in re.captures_iter(&line) {
            let num1: i32 = mult[1].parse().unwrap();
            let num2: i32 = mult[2].parse().unwrap();

            mults.push(num1*num2);
        }
    }

    let result: i32 = mults.iter().sum();
    println!("{}", result);
    Ok(())
}