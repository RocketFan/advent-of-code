use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use day06::*;

enum ReadPhase {
    ReadNumbers,
    ReadSymbols,
}

fn read_numbers_and_symbols(filepath: &Path) -> (Vec<Vec<i64>>, Vec<Symbol>) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut read_phase = ReadPhase::ReadNumbers;
    let elements_num = reader.lines().next().unwrap().unwrap().split_whitespace().count();
    let mut numbers = vec![Vec::new(); elements_num];
    let mut symbols = Vec::new();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let line_elements: Vec<&str> = line.split_whitespace().collect();
        
        if line_elements[0] == "*" || line_elements[0] == "+" { read_phase = ReadPhase::ReadSymbols }

        match read_phase {
            ReadPhase::ReadNumbers => for (i, element) in line_elements.iter().enumerate() { numbers[i].push(element.parse().unwrap()) },
            ReadPhase::ReadSymbols => symbols = line_elements.into_iter().map(|x| x.parse().unwrap()).collect(),
        }
    }

    (numbers, symbols)
}

fn main() {
    let filepath = Path::new("day06/input.txt");
    let (numbers, symbols) = read_numbers_and_symbols(filepath);
    println!("Numbers: {:?}", numbers);
    println!("Symbols: {:?}", symbols);
    let sum_results = calculate_sum_of_results(numbers, symbols); 
    println!("Sum of results: {}", sum_results);
}