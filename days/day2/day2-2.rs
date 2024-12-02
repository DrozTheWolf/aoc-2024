use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut safe_reports: i32 = 0; 

    let file = File::open("./days/day2/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        let levels: Vec<i32> = parts.iter().map(|x| x.parse().unwrap()).collect();

        if analyze_report(levels) {
            safe_reports += 1;
        }
    }
    
    println!("{}", safe_reports);
    Ok(())
}

fn analyze_report(levels: Vec<i32>) -> bool {
    if check_safety(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut new_levels = levels.clone();
        new_levels.remove(i);
        if check_safety(&new_levels) {
            return true;
        }
    }

    false
}

fn check_safety(levels: &Vec<i32>) -> bool {
    let mut is_ascending = true;
    let mut is_descending = true;

    for i in 1..levels.len() {
        let diff = (levels[i] - levels[i-1]).abs();

        if diff == 0 || diff > 3 {
            return false
        }

        if levels[i] > levels[i-1] {
            is_descending = false;
        } else {
            is_ascending = false;
        }
    }

    is_ascending || is_descending

}