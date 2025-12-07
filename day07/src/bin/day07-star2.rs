use day07::*;
use std::{collections::HashMap, path::Path};

fn count_possible_paths(
    manifold: &Vec<Vec<ManifoldField>>,
    lookup: &mut HashMap<(usize, usize), usize>,
    row: usize,
    col: usize,
) -> usize {
    if row >= manifold.len() {
        return 1;
    }

    if let Some(count) = lookup.get(&(row, col)) {
        return *count;
    }

    match manifold[row][col] {
        ManifoldField::Splitter => {
            let left = count_possible_paths(manifold, lookup, row, col - 1);
            let right = count_possible_paths(manifold, lookup, row, col + 1);

            lookup.insert((row, col), left + right);

            return left + right;
        }
        _ => count_possible_paths(manifold, lookup, row + 1, col),
    }
}

fn main() {
    let filepath = Path::new("day07/input.txt");
    let (mut manifold, beam_position) = read_tachyon_manifold_with_beam_position(filepath);
    println!("Tachyon manifold: {:?}", manifold);
    let _ = draw_beams_and_calculate_splits(&mut manifold);
    println!("Changed tachyon manifold: {:?}", manifold);
    let possible_paths = count_possible_paths(&manifold, &mut HashMap::new(), 0, beam_position);
    println!("Number of possible paths: {}", possible_paths)
}
