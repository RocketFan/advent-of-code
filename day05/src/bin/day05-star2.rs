use std::path::Path;
use day05::*;

fn combine_overlapping_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut combined_ranges = Vec::new();
    while !ranges.is_empty() {
        let mut last_size: i64 = -1;

        let mut range = ranges.pop().unwrap();

        while ranges.len() as i64 != last_size {
            last_size = ranges.len() as i64;

            ranges.retain(|&other_range| {
                let other_range = other_range.0..other_range.1 + 1;
                let range_bounds = range.0..range.1 + 1;
                if other_range.contains(&range.0) || other_range.contains(&range.1) || range_bounds.contains(&other_range.start) || range_bounds.contains(&(other_range.end - 1)) {
                    let other_range_min = other_range.clone().min().unwrap();
                    let other_range_max = other_range.max().unwrap();
                    range = (range.0.min(other_range_min), range.1.max(other_range_max));
                    return false
                }
                
                true
            }); 
        }

        combined_ranges.push(range);
    }

    combined_ranges
}

fn count_all_fresh_ingredients(fresh_ranges: &Vec<(usize, usize)>) -> usize {
    let mut all_fresh_num = 0;

    for fresh_range in fresh_ranges {
        all_fresh_num += fresh_range.1 - fresh_range.0 + 1;
    }

    all_fresh_num
}

fn main() {
    let filepath = Path::new("day05/input.txt");
    let (_, fresh_ranges) = read_available_ingredients_with_fresh_ranges(filepath);
    println!("Fresh ranges {}: {:?}", fresh_ranges.len(), fresh_ranges);
    let combined_fresh_ranges = combine_overlapping_ranges(fresh_ranges.clone());
    println!("Combined fresh ranges {}: {:?}", combined_fresh_ranges.len(), combined_fresh_ranges);
    let number_of_all_fresh_ingredients = count_all_fresh_ingredients(&combined_fresh_ranges);
    println!("All fresh ingredients: {}", number_of_all_fresh_ingredients);
}