use std::fs::File;
use std::io::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("inputs/day5.txt")?;
    let reader = std::io::BufReader::new(input_file);
    let mut max_seat_id = 0;
    let mut seat_ids = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let seat_id = get_seat_id(l);
        seat_ids.push(seat_id);
        if seat_id > max_seat_id {
            max_seat_id = seat_id
        }
    }

    seat_ids.sort();
    let mut a = seat_ids[0] - 1;
    for i in seat_ids {
        if i - a != 1 {
            println!("Your seatid is {}", i);
            a = i;
        } else {
            a = i
        }
    }
    println!("max seat id is {}", max_seat_id);

    Ok(())
}
fn get_seat_id(input: String) -> u64 {
    let mut lower_range: u64 = 0;
    let mut higher_range: u64 = 127;
    let mut lower_seat: u64 = 0;
    let mut higher_seat: u64 = 7;
    for char in input.chars().into_iter() {
        match char {
            'F' => {
                if (lower_range + higher_range) % 2 == 0 {
                    higher_range = (lower_range + higher_range) / 2 + 1;
                } else {
                    higher_range = (lower_range + higher_range) / 2;
                }
            }
            'B' => {
                if (lower_range + higher_range) % 2 == 0 {
                    lower_range = (lower_range + higher_range) / 2;
                } else {
                    lower_range = (lower_range + higher_range) / 2 + 1;
                }
            }
            'L' => {
                if (lower_seat + higher_seat) % 2 == 0 {
                    higher_seat = (lower_seat + higher_seat) / 2 + 1;
                } else {
                    higher_seat = (lower_seat + higher_seat) / 2;
                }
            }
            'R' => {
                if (lower_seat + higher_seat) % 2 == 0 {
                    lower_seat = (lower_seat + higher_seat) / 2;
                } else {
                    lower_seat = (lower_seat + higher_seat) / 2 + 1;
                }
            }
            _ => {
                panic!("unknown character")
            }
        }
    }
    return lower_range * 8 + lower_seat;
}
