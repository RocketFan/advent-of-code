use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const AVAILABLE_MOVES: [(i64, i64); 8] = [(1, 0), (1, 1), (0, 1), (-1, 0), (-1, -1), (0, -1), (-1, 1), (1, -1)];

fn read_shelves_matrix(filepath: &Path) -> Vec<Vec<bool>> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|row| row.unwrap().chars().map(|c| c == '@').collect()).collect()
}

fn count_neighbors(shelves: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut neighbors_num = 0;

    for (move_row, move_col) in AVAILABLE_MOVES {
        let neighbor_row = row as i64 + move_row;
        let neighbor_col = col as i64 + move_col;

        if neighbor_row < 0 || neighbor_col < 0 || neighbor_row >= shelves.len() as i64 || neighbor_col >= shelves[row].len() as i64 { continue }

        let neighbor = shelves[neighbor_row as usize][neighbor_col as usize];

        if neighbor { neighbors_num += 1 }
    }

    neighbors_num
}

fn number_of_accessible_items_on_shelves(shelves: &Vec<Vec<bool>>) -> usize{
    let mut number_of_accessible_items = 0;

    for i in 0..shelves.len() {
        for j in 0..shelves[i].len() {
            if !shelves[i][j] { continue }

            let neighbors = count_neighbors(shelves, i, j);

            if neighbors < 4 { number_of_accessible_items += 1 }
        }
    }

    number_of_accessible_items
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = Path::new("day04/input.txt");
    let shelves = read_shelves_matrix(filepath);
    println!("{:?}", shelves);
    let number_of_accessible_items = number_of_accessible_items_on_shelves(&shelves);
    println!("Number of accessible items on the shelve: {}", number_of_accessible_items);
    Ok(())
}