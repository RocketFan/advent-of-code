use std::path::Path;
use std::error::Error;
use day03::*;

fn find_biggest_joltage(battery_bank: &Vec<usize>) -> usize{
    let mut max_first_idx: usize = 0;
    let mut max_first_value: usize = 0;

    for (i, value) in battery_bank.iter().enumerate() {
        if value > &max_first_value && i != battery_bank.len() - 1 {
           max_first_idx = i; 
           max_first_value = *value;
        }
    }

    let max_second_value = battery_bank.iter().skip(max_first_idx + 1).max().unwrap();
    
    (max_first_value.to_string() + &max_second_value.to_string()).parse().unwrap()
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