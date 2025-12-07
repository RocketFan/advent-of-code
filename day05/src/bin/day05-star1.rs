use std::{ops::Range, path::Path};
use day05::*;

fn count_fresh_ingredients(ingredients: &Vec<usize>, fresh_ranges: &Vec<Range<usize>>) -> usize {
    let mut fresh_num = 0;

    for ingredient in ingredients {
        for fresh_range in fresh_ranges {
            if fresh_range.contains(ingredient) { 
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