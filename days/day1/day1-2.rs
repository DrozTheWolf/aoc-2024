use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut result_list: Vec<i32> = Vec::new();

    let file = File::open("./days/day1/inputs/input1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let num1: i32 = parts[0].parse().unwrap();
        let num2: i32 = parts[1].parse().unwrap();

        list1.push(num1);
        list2.push(num2);
    }


    for &num in &list1 {
        let mut count = 0;
        for &num2 in &list2 {
            if num2 == num {
                count += 1;
            }
        }
        result_list.push(count * num);
    }

    let result: i32 = result_list.iter().sum();

    println!("{}", result);

    Ok(())
}