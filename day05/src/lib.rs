use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;

pub enum ReadPhase {
    FreshRanges,
    BlankLine,
    AvailableIngredients
}

pub fn read_available_ingredients_with_fresh_ranges(filepath: &Path) -> (Vec<usize>, Vec<(usize, usize)>) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut fresh_ranges = Vec::new();
    let mut available_ingredients = Vec::new();
    let mut read_phase = ReadPhase::FreshRanges;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() { read_phase = ReadPhase::BlankLine }

        match read_phase {
            ReadPhase::FreshRanges => {
                let range: (usize, usize) = line.split("-").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
                if range.0 > range.1 {
                    panic!("Range max {} lower than range min {}", range.1, range.0)
                }
                fresh_ranges.push(range);
            },
            ReadPhase::BlankLine => read_phase = ReadPhase::AvailableIngredients,
            ReadPhase::AvailableIngredients => available_ingredients.push(line.parse().unwrap()),
        }
    }

    (available_ingredients, fresh_ranges)
}
