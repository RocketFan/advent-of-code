
#[derive(PartialEq, Debug, Clone)]
pub enum RotationCommand {
    RotateLeft(i64),
    RotateRight(i64),
}

pub fn deserialize_rotation_command(cmd: &str) -> RotationCommand {
    let rotation_char = cmd.chars().nth(0).unwrap();
    match rotation_char {
        'L' => RotationCommand::RotateLeft(cmd[1..].parse().unwrap()),
        'R' => RotationCommand::RotateRight(cmd[1..].parse().unwrap()),
        _ => panic!("Rotation char not supported {}", rotation_char),
    }
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