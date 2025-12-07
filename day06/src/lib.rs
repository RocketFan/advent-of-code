use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Symbol {
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

pub fn calculate_sum_of_results(numbers: Vec<Vec<i64>>, symbols: Vec<Symbol>) -> i64 {
    let mut sum_results = 0;
    for col in 0..numbers.len() {
        let mut result = 0;
        let symbol = symbols[col].clone();
        for row in 0..numbers[col].len() {
            let number = numbers[col][row];
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

