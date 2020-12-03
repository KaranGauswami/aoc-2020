use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("inputs/day3.txt")?;
    let reader = std::io::BufReader::new(input_file);

    let mut square: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let array: Vec<char> = l.chars().collect();
        square.push(array);
    }
    let row_len = square[0].len();
    let mut x_slope1 = 0;
    let mut x_slope2 = 0;
    let mut x_slope3 = 0;
    let mut x_slope4 = 0;
    let mut x_slope5 = 0;
    let mut y_pos5 = 0;
    let mut slope1_trees = 0;
    let mut slope2_trees = 0;
    let mut slope3_trees = 0;
    let mut slope4_trees = 0;
    let mut slope5_trees = 0;
    for y_pos in 0..square.len() {
        if x_slope2 >= row_len {
            x_slope2 = x_slope2 - row_len;
        }
        if x_slope1 >= row_len {
            x_slope1 = x_slope1 - row_len;
        }
        if x_slope3 >= row_len {
            x_slope3 = x_slope3 - row_len;
        }
        if x_slope4 >= row_len {
            x_slope4 = x_slope4 - row_len;
        }
        if x_slope5 >= row_len {
            x_slope5 = x_slope5 - row_len;
        }
        if square[y_pos][x_slope1] == '#' {
            slope1_trees += 1;
        }
        if square[y_pos][x_slope2] == '#' {
            slope2_trees += 1;
        }
        if square[y_pos][x_slope3] == '#' {
            slope3_trees += 1;
        }
        if square[y_pos][x_slope4] == '#' {
            slope4_trees += 1;
        }
        if y_pos5 < square.len() && square[y_pos5][x_slope5] == '#' {
            slope5_trees += 1;
        }
        x_slope1 += 1;
        x_slope2 += 3;
        x_slope3 += 5;
        x_slope4 += 7;
        x_slope5 += 1;
        y_pos5 += 2;
    }
    println!("Total trees of part 1 are {}", slope2_trees);
    let total_trees: u64 = slope1_trees * slope2_trees * slope3_trees * slope4_trees * slope5_trees;
    println!("Total trees of part 2 are {}", total_trees);
    Ok(())
}
