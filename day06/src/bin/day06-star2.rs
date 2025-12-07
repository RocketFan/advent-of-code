use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use day06::*;

fn read_numbers_and_symbols(filepath: &Path) -> (Vec<Vec<i64>>, Vec<Symbol>) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let last_line = reader.lines().last().unwrap().unwrap();
    let mut symbols = vec![last_line.chars().next().unwrap().to_string().parse().unwrap()];
    let mut column_sizes = Vec::new();

    let mut column_size = 1;
    for char in last_line.chars().skip(1) {
        if let Ok(symbol) = char.to_string().parse() {
            symbols.push(symbol);
            column_sizes.push(column_size - 1);
            column_size = 0;
        }

        column_size += 1;
    }
    column_sizes.push(column_size);
    println!("Column sizes: {:?}", column_sizes);

    let mut numbers = vec![Vec::new(); symbols.len()];
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let line = line.unwrap();
        if line.contains(&['*', '+']) { break }

        let mut pointer = 0;
        for (i, size) in column_sizes.iter().enumerate() {
            numbers[i].push(line[pointer..pointer+size].to_string());
            pointer += size + 1;
            println!("Last number: {}", numbers[i].last().unwrap())
        }
    }
    println!("Numbers: {:?}", numbers);
    let mut column_numbers: Vec<Vec<i64>> = Vec::new();

    for (col, size) in column_sizes.iter().enumerate() {
        let mut column_elements = vec![String::new(); *size];

        for number in &numbers[col] {
            for (i, num) in number.chars().enumerate() { column_elements[i] += &num.to_string() }
        }
        column_numbers.push(column_elements.iter().map(|x| x.replace(" ", "").parse().unwrap()).collect());
        println!("Column numbers: {:?}", column_numbers.last().unwrap());
    }

    (column_numbers, symbols)
}

fn main() {
    let filepath = Path::new("day06/input.txt");
    let (numbers, symbols) = read_numbers_and_symbols(filepath);
    println!("Numbers: {:?}", numbers);
    println!("Symbols: {:?}", symbols);
    let sum_results = calculate_sum_of_results(numbers, symbols); 
    println!("Sum of results: {}", sum_results);
}