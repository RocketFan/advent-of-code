use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;

enum ReadPhase {
    FreshRanges,
    BlankLine,
    AvailableIngredients
}

fn read_available_ingredients_with_fresh_ranges(filepath: &Path) -> (Vec<usize>, Vec<(usize, usize)>) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut fresh_ranges = Vec::new();
    let mut available_ingredients = Vec::new();
    let mut read_phase = ReadPhase::FreshRanges;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() { read_phase = ReadPhase::BlankLine }

        match read_phase {
            ReadPhase::FreshRanges => fresh_ranges.push(line.split("-").map(|s| s.parse().unwrap()).collect_tuple().unwrap()),
            ReadPhase::BlankLine => read_phase = ReadPhase::AvailableIngredients,
            ReadPhase::AvailableIngredients => available_ingredients.push(line.parse().unwrap()),
        }
    }

    (available_ingredients, fresh_ranges)
}

fn count_fresh_ingredients(ingredients: &Vec<usize>, fresh_ranges: &Vec<(usize, usize)>) -> usize {
    let mut fresh_num = 0;

    for ingredient in ingredients {
        for fresh_range in fresh_ranges {
            if *ingredient >= fresh_range.0 && *ingredient <= fresh_range.1 { 
                fresh_num += 1;
                break;
            }    
        }
    }

    fresh_num
}

fn main() {
    let filepath = Path::new("day05/input.txt");
    let (available_ingredients, fresh_ranges) = read_available_ingredients_with_fresh_ranges(filepath);
    println!("Available ingredients: {:?}", available_ingredients);
    println!("Fresh ranges: {:?}", fresh_ranges);
    let number_of_fresh_ingredients = count_fresh_ingredients(&available_ingredients, &fresh_ranges);
    println!("Fresh ingredients: {}", number_of_fresh_ingredients);
}