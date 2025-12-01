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
            RotationCommand::RotateLeft(n_times) => { 
                let rotated_dial = safe_dial - n_times;
                if rotated_dial < 0 && safe_dial != 0 {
                    count_safe_dial_zero_position += 1;
                    count_safe_dial_zero_position -= if rotated_dial % 100 == 0 {1} else {0};
                }

                count_safe_dial_zero_position += (rotated_dial / 100).abs();
                // println!("Rotated dial {} divided {}", rotated_dial, (rotated_dial / 100).abs());
                
                rotated_dial.rem_euclid(100)
             },
            RotationCommand::RotateRight(n_times) => {
                let rotated_dial = safe_dial + n_times;
                count_safe_dial_zero_position += (rotated_dial / 100).abs();
                count_safe_dial_zero_position -= if rotated_dial % 100 == 0 {1} else {0};
                rotated_dial.rem_euclid(100)
            },
        };

        let safe_dial_position = safe_dial;

        if safe_dial_position == 0 {
            count_safe_dial_zero_position += 1;
        }

        println!("Rotated safe dial {:?} to position {}", cmd, safe_dial_position);
        println!("Zero positions count {}", count_safe_dial_zero_position);
    }

    println!("Number of safe dial zero positions during the sequence was {}", count_safe_dial_zero_position);

    Ok(())
}