use day07::*;
use std::path::Path;

fn main() {
    let filepath = Path::new("day07/input.txt");
    let (mut manifold, _) = read_tachyon_manifold_with_beam_position(filepath);
    println!("Tachyon manifold: {:?}", manifold);
    let splits = draw_beams_and_calculate_splits(&mut manifold);
    println!("Number of beam splits: {}", splits)
}
