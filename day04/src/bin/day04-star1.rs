use std::error::Error;
use std::path::Path;
use day04::*;

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