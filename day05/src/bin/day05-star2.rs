use day05::*;
use std::{ops::Range, path::Path};

fn combine_overlapping_ranges(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut combined_ranges = Vec::new();
    while !ranges.is_empty() {
        let mut last_size: i64 = -1;

        let mut range = ranges.pop().unwrap();

        while ranges.len() as i64 != last_size {
            last_size = ranges.len() as i64;

            ranges.retain(|other_range| {
                if other_range.contains(&range.start)
                    || other_range.contains(&(range.end - 1))
                    || range.contains(&other_range.start)
                    || range.contains(&(other_range.end - 1))
                {
                    range = range.start.min(other_range.start)..(range.end).max(other_range.end);
                    return false;
                }

                true
            });
        }

        combined_ranges.push(range);
    }

    combined_ranges
}

fn count_all_fresh_ingredients(fresh_ranges: &Vec<Range<usize>>) -> usize {
    let mut all_fresh_num = 0;

    for fresh_range in fresh_ranges {
        all_fresh_num += fresh_range.end - fresh_range.start;
    }

    all_fresh_num
}

fn main() {
    let filepath = Path::new("day05/input.txt");
    let (_, fresh_ranges) = read_available_ingredients_with_fresh_ranges(filepath);
    println!("Fresh ranges {}: {:?}", fresh_ranges.len(), fresh_ranges);
    let combined_fresh_ranges = combine_overlapping_ranges(fresh_ranges.clone());
    println!(
        "Combined fresh ranges {}: {:?}",
        combined_fresh_ranges.len(),
        combined_fresh_ranges
    );
    let number_of_all_fresh_ingredients = count_all_fresh_ingredients(&combined_fresh_ranges);
    println!("All fresh ingredients: {}", number_of_all_fresh_ingredients);
}
