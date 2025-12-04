use std::error::Error;
use itertools::Itertools;

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

    if length % 2 != 0 {
        return false;
    }

    let middle = length / 2;

    number[..middle] == number[middle..]
}

fn read_ranges_from_file(filepath: &std::path::Path) -> Vec<(usize, usize)> {
    let input = std::fs::read_to_string(filepath).unwrap();
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for range in input.split(",") {
        if range.is_empty() {
            continue;
        }

        let range: (usize, usize) = range.split("-").map(|x| x.parse().unwrap()).collect_tuple().unwrap();
        ranges.push(range);
    }

    ranges
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
        assert!(is_doubled_sequence(5050));
        assert!(is_doubled_sequence(123123));
    }

    #[test]
    fn double_sequence_not_detected_in_odd_length_number() {
        assert!(!is_doubled_sequence(1));
        assert!(!is_doubled_sequence(111));
    }
}
