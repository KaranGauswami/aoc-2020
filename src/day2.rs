use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("day2.txt")?;
    let reader = std::io::BufReader::new(input_file);

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut valid_passwords: u64 = 0;
    let mut valid_passwords_part2: u64 = 0;

    for line in reader.lines() {
        let l = line?;
        let caps = re.captures(&l).unwrap();
        let mins = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let maxs = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let character = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();
        let count = password.matches(character).count() as u8;
        if mins <= count && count <= maxs {
            valid_passwords += 1;
        }
        let char1 = password.chars().nth((mins - 1).into());
        let char2 = password.chars().nth((maxs - 1).into());
        if char1 != char2
            && (character.chars().nth(0) == char1 || character.chars().nth(0) == char2)
        {
            valid_passwords_part2 += 1;
        }
    }
    println!("valid passwords for part01 are : {}", valid_passwords);
    println!("valid passwords for part02 are : {}", valid_passwords_part2);

    Ok(())
}
