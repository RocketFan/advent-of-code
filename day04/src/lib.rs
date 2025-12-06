use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub const AVAILABLE_MOVES: [(i64, i64); 8] = [(1, 0), (1, 1), (0, 1), (-1, 0), (-1, -1), (0, -1), (-1, 1), (1, -1)];

pub fn read_shelves_matrix(filepath: &Path) -> Vec<Vec<bool>> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|row| row.unwrap().chars().map(|c| c == '@').collect()).collect()
}

pub fn count_neighbors(shelves: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
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
