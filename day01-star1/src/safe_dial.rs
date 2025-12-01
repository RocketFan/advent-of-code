pub struct SafeDial {
    range: i64,
    position: i64,
}

impl SafeDial {
    pub fn new(range: i64, position: i64) -> Self {
        Self {
            range,
            position,
        }
    }

    pub fn rotate_right(&mut self, n_times: i64) {
        let rotated_position = self.position + n_times;

        if rotated_position > self.range {
            self.position = rotated_position % (self.range + 1);
        } else {
            self.position = rotated_position;
        }
    }

    pub fn rotate_left(&mut self, n_times: i64) {
        let rotated_position = self.position - n_times;
        
        if rotated_position < 0 {
            let rotated_position = rotated_position % (self.range + 1);
            self.position = if rotated_position == 0 {0} else {self.range + 1 + rotated_position};
        } else {
            self.position = rotated_position;
        }

    }

    pub fn get_position(&self) -> i64 {
        return self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_rotation_cycle() {
        let mut safe_dial = SafeDial::new(10, 0);
        safe_dial.rotate_left(1);
        assert_eq!(safe_dial.get_position(), 10);
    }

    #[test]
    fn test_left_rotation_full_cycle() {
        let mut safe_dial = SafeDial::new(10, 0);
        safe_dial.rotate_left(11);
        assert_eq!(safe_dial.get_position(), 0)
    }

    #[test]
    fn test_left_rotation_double_cycle() {
        let mut safe_dial = SafeDial::new(10, 10);
        safe_dial.rotate_left(33);
        assert_eq!(safe_dial.get_position(), 10);
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
        safe_dial.rotate_right(11);
        assert_eq!(safe_dial.get_position(), 0);
    }

    #[test]
    fn test_right_rotation_double_cycle() {
        let mut safe_dial = SafeDial::new(10, 0);
        safe_dial.rotate_right(33);
        assert_eq!(safe_dial.get_position(), 0);
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