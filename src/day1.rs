use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("inputs/day1.txt")?;
    let reader = std::io::BufReader::new(input_file);

    let mut numbers = Vec::new();
    for line in reader.lines() {
        let digit = line?.parse::<u32>().unwrap();
        numbers.push(digit)
    }

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            for k in j..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("Part 2 answer : {} ", numbers[i] * numbers[j] * numbers[k]);
                }
            }
            if numbers[i] + numbers[j] == 2020 {
                println!("Part 1 answer : {} ", numbers[i] * numbers[j]);
            }
        }
    }

    Ok(())
}
