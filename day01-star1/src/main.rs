struct SafeDial {
    range: i64,
    position: i64,
}

impl SafeDial {
    fn new(range: i64, position: i64) -> Self {
        Self {
            range,
            position,
        }
    }

    fn rotate_right(&mut self, n_times: i64) {
        let rotated_position = self.position + n_times;

        if rotated_position > self.range {
            self.position = rotated_position % self.range - 1;
        } else {
            self.position = rotated_position;
        }
    }

    fn rotate_left(&mut self, n_times: i64) {
        let rotated_position = self.position - n_times;
        
        if rotated_position < 0 {
            self.position = (self.range + rotated_position % self.range) + 1;
        } else {
            self.position = rotated_position;
        }

    }

    fn get_position(&self) -> i64 {
        return self.position
    }
}

fn main() {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_rotation_cycle() {
        let mut safe_dial = SafeDial::new(10, 0);
        safe_dial.rotate_left(2);
        assert_eq!(safe_dial.get_position(), 9);
    }

    #[test]
    fn test_left_rotation_without_cycle() {
        let mut safe_dial = SafeDial::new(10, 10);
        safe_dial.rotate_left(5);
        assert_eq!(safe_dial.get_position(), 5);
    }

    #[test]
    fn test_right_rotation_cycle() {
        let mut safe_dial = SafeDial::new(10, 0);
        safe_dial.rotate_right(12);
        assert_eq!(safe_dial.get_position(), 1);
    }

    #[test]
    fn test_right_rotation_without_cycle() {
        let mut safe_dial = SafeDial::new(10, 0);
        safe_dial.rotate_right(5);
        assert_eq!(safe_dial.get_position(), 5);
    }

    #[test]
    fn test_sequence() {
        let mut safe_dial = SafeDial::new(99, 50);
        safe_dial.rotate_left(68);
        safe_dial.rotate_left(30);
        safe_dial.rotate_right(48);
        safe_dial.rotate_left(5);
        safe_dial.rotate_right(60);
        safe_dial.rotate_left(55);
        safe_dial.rotate_left(1);
        safe_dial.rotate_left(99);
        safe_dial.rotate_right(14);
        safe_dial.rotate_left(82);
        assert_eq!(safe_dial.get_position(), 32);
    }
}