use std::{fs::File, io::{BufRead, BufReader}};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mults: Vec<i32> = Vec::new();

    let file = File::open("./days/day3/input.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    let mut activated = true;

    for line in reader.lines() {
        let line = line?;
        for capture in re.captures_iter(&line) {
            if let Some(_mult) = capture.get(1) {
                if activated {
                    let num1: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                    let num2: i32 = capture.get(3).unwrap().as_str().parse().unwrap();
                    mults.push(num1*num2);
                }
            } else if let Some(_do_group) = capture.get(4) {
                activated = true;
            } else if let Some(_dont) = capture.get(5) {
                activated = false;
            }
            
        }
    }

    let result: i32 = mults.iter().sum();
    println!("{}", result);

    Ok(())
}