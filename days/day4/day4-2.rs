use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut count: i32 = 0;

    let file = File::open("./days/day4/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
       let line = line?;
       let line_chars: Vec<char> = line.chars().collect();
       grid.push(line_chars);
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'A' {
                if search(&grid, i as isize, j as isize) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn search(grid: &[Vec<char>], x: isize, y: isize) -> bool {
    let top_x = x - 1;
    let bot_x = x + 1;
    let left_y = y - 1;
    let right_y = y + 1;

    if top_x < 0 || bot_x as usize >= grid.len() || left_y < 0 || right_y as usize >= grid[0].len() {
        return false; 
    } else {
        let top_left: char = grid[top_x as usize][left_y as usize];
        let top_right:char = grid[top_x as usize][right_y as usize];
        let bot_left:char = grid[bot_x as usize][left_y as usize];
        let bot_right: char = grid[bot_x as usize][right_y as usize];

        if (top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M') {
            if (top_right == 'M' && bot_left == 'S') || (top_right == 'S' && bot_left == 'M') {
                return true;
            }
        }
    }

    false
}
