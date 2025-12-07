use std::{fs::File, io::{BufRead, BufReader}, path::Path, str::FromStr};

enum ReadPhase {
    ReadNumbers,
    ReadSymbols,
}

#[derive(Clone, Debug)]
enum Symbol {
    Add,
    Multiply
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Symbol::Multiply),
            "+" => Ok(Symbol::Add),
            _ => Err(format!("Unknown symbol {}", s))
        }
    }
}

fn read_numbers_and_symbols(filepath: &Path) -> (Vec<Vec<i64>>, Vec<Symbol>) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut read_phase = ReadPhase::ReadNumbers;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line_elements: Vec<&str> = line.split_whitespace().collect();
        
        if line_elements[0] == "*" || line_elements[0] == "+" { read_phase = ReadPhase::ReadSymbols }

        match read_phase {
            ReadPhase::ReadNumbers => numbers.push(line_elements.into_iter().map(|x| x.parse().unwrap()).collect()),
            ReadPhase::ReadSymbols => symbols = line_elements.into_iter().map(|x| x.parse().unwrap()).collect(),
        }
    }

    (numbers, symbols)
}

fn calculate_sum_of_results(numbers: Vec<Vec<i64>>, symbols: Vec<Symbol>) -> i64 {
    let mut sum_results = 0;
    for col in 0..numbers[0].len() {
        let mut result = 0;
        let symbol = symbols[col].clone();
        for row in 0..numbers.len() {
            let number = numbers[row][col];
            match symbol {
                Symbol::Add => result += number,
                Symbol::Multiply => if result == 0 { result = number } else { result *= number },
            }
        }
        sum_results += result;
        println!("Result: {}", result);
    }
    sum_results
}

fn main() {
    let filepath = Path::new("day06/input.txt");
    let (numbers, symbols) = read_numbers_and_symbols(filepath);
    println!("Numbers: {:?}", numbers);
    println!("Symbols: {:?}", symbols);
    let sum_results = calculate_sum_of_results(numbers, symbols); 
    println!("Sum of results: {}", sum_results);
}