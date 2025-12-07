use std::path::Path;
use day05::*;

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