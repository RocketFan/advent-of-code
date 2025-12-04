use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn read_battery_banks(filepath: &Path) -> Vec<Vec<usize>> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|s| s.unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}
