use std::path::Path;
use std::error::Error;
use itertools::Itertools;
use day03::*;

fn find_largest_number_before<'a>(values: impl Iterator<Item = &'a usize>, stop_idx: usize) -> (usize, usize){
    let mut max_idx: usize = 0;
    let mut max_value: usize = 0;

    for (i, value) in values.enumerate() {
        if stop_idx == i { break }

        if value > &max_value {
           max_idx = i; 
           max_value = *value;
        }
    }

    (max_idx, max_value) 
}

fn find_biggest_joltage(battery_bank: &Vec<usize>) -> usize{
    let mut batteries = Vec::new();
    let mut skip_idx = 0;

    for i in (0..12).rev() {
        let (idx, voltage) = find_largest_number_before(battery_bank.iter().skip(skip_idx), battery_bank.len() - i - skip_idx);
        skip_idx += idx + 1;
        batteries.push(voltage);
    }
    
    batteries.iter().join("").parse().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = Path::new("day03/input.txt");
    let battery_banks = read_battery_banks(filepath);
    println!("Battery banks: {:?}", battery_banks);
    println!("Battery bank size: {}", battery_banks[0].len());

    let mut battery_joltage_sum = 0;
    for battery_bank in battery_banks {
        let max_battery_bank_joltage = find_biggest_joltage(&battery_bank);
        battery_joltage_sum += max_battery_bank_joltage;
        println!("{}", max_battery_bank_joltage);
    }

    println!("Battery joltage: {}", battery_joltage_sum);

    Ok(())
}