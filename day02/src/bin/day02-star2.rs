use std::error::Error;
use day02::*;

fn invalid_ids_in_range(min: usize, max: usize) -> Vec<usize> {
    let mut invalid_ids = Vec::new();

    for number in min..max+1 {
        if is_doubled_sequence(number) {
            invalid_ids.push(number);
        }
    }

    invalid_ids
}

fn is_doubled_sequence(number: usize) -> bool {
    let number = number.to_string();
    let length = number.len();

    for i in 1..length / 2 + 1 {
        if length % i != 0 { continue };
        let window = &number[..i];
        let mut found_sequence = true;

        for j in (0..length).step_by(i) { 
            let step_window = &number[j..j+i];
            if window != step_window {
                found_sequence = false;
                break;
            }
        }

        if found_sequence { return true }
    }

    return false;
}

fn main() -> Result<(), Box<dyn Error>>{
    let filepath = std::path::Path::new("day02/input.txt");
    let ranges = read_ranges_from_file(filepath);
    let mut invalid_ids_sum: usize = 0;

    for (min, max) in ranges {
        let invalid_ids = invalid_ids_in_range(min, max);
        println!("Detected invalid ids {:?}", invalid_ids);
        invalid_ids_sum += invalid_ids.iter().sum::<usize>();
    }

    println!("Sum of detected invalid ids {}", invalid_ids_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_sequence_detected() {
        assert!(is_doubled_sequence(11));
        assert!(is_doubled_sequence(505050));
        assert!(is_doubled_sequence(123123));
    }

    #[test]
    fn double_sequence_detected_in_odd_length_number() {
        assert!(is_doubled_sequence(131131131));
        assert!(is_doubled_sequence(111));
    }
}
