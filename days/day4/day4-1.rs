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
            if grid[i][j] == 'X' {
                count += search_in_all_directions(&grid, i as isize, j as isize);
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn search(grid: &[Vec<char>], x: isize, y: isize, dx: isize, dy: isize) -> bool {
    let final_x = x+3*dx;
    let final_y = y+3*dy;
    if final_x < 0 || final_y <0 || final_x >= grid.len() as isize || final_y >= grid[0].len() as isize {
        return false;
    } else {
         return grid[(x + dx) as usize][(y + dy) as usize] == 'M' &&
         grid[(x + 2*dx) as usize][(y + 2*dy) as usize] == 'A' && grid[final_x as usize][final_y as usize] == 'S'
    }

}

fn search_in_all_directions(grid: &[Vec<char>], x: isize, y: isize) -> i32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if !(i == 0 && j == 0) {
                if search(grid, x, y, i, j) {
                    count += 1;
                }
            }
        }
    }

    count
}
