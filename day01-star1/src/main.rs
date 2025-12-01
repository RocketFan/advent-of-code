use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod safe_dial;
use safe_dial::*;

#[derive(PartialEq, Debug, Clone)]
enum RotationCommand {
    RotateLeft(i64),
    RotateRight(i64),
}

fn deserialize_rotation_command(cmd: &str) -> RotationCommand {
    let rotation_char = cmd.chars().nth(0).unwrap();
    match rotation_char {
        'L' => RotationCommand::RotateLeft(cmd[1..].parse().unwrap()),
        'R' => RotationCommand::RotateRight(cmd[1..].parse().unwrap()),
        _ => panic!("Rotation char not supported {}", rotation_char),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut safe_dial = SafeDial::new(99, 50);
    let file = File::open("day01-star1\\input.txt")?;
    let reader = BufReader::new(file);

    let mut count_safe_dial_zero_position = 0;

    for line in reader.lines() {
        let line = line?;
        let cmd = deserialize_rotation_command(&line);

        match cmd {
            RotationCommand::RotateLeft(n_times) => safe_dial.rotate_left(n_times),
            RotationCommand::RotateRight(n_times) => safe_dial.rotate_right(n_times),
        }

        let safe_dial_position = safe_dial.get_position();

        if safe_dial_position == 0 {
            count_safe_dial_zero_position += 1;
        }

        println!("Rotated safe dial {:?} to position {}", cmd, safe_dial_position);
    }

    println!("Number of safe dial zero positions during the sequence was {}", count_safe_dial_zero_position);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_left_command() {
        let cmd = "L648";
        let cmd = deserialize_rotation_command(cmd);
        assert_eq!(cmd, RotationCommand::RotateLeft(648))
    }

    #[test]
    fn test_deserialize_right_command() {
        let cmd = "R847";
        let cmd = deserialize_rotation_command(cmd);
        assert_eq!(cmd, RotationCommand::RotateRight(847))
    }
}