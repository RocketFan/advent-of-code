use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use day01::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut safe_dial = 50;
    let file = File::open("day01\\input.txt")?;
    let reader = BufReader::new(file);

    let mut count_safe_dial_zero_position = 0;

    for line in reader.lines() {
        let line = line?;
        let cmd = deserialize_rotation_command(&line);

        safe_dial = match cmd {
            RotationCommand::RotateLeft(n_times) => (safe_dial - n_times).rem_euclid(100),
            RotationCommand::RotateRight(n_times) => (safe_dial + n_times).rem_euclid(100),
        };

        let safe_dial_position = safe_dial;

        if safe_dial_position == 0 {
            count_safe_dial_zero_position += 1;
        }

        println!("Rotated safe dial {:?} to position {}", cmd, safe_dial_position);
    }

    println!("Number of safe dial zero positions during the sequence was {}", count_safe_dial_zero_position);

    Ok(())
}