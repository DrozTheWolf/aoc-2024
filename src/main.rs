//TEMPLATE FOR DAILY STARTER
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("./days/dayX/input.txt")?;
    let reader = BufReader::new(file);
    
    Ok(())
}
