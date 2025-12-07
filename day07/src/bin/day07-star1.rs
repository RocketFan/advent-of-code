use std::path::Path;

fn read_tachyon_manifold_and_beam_position(filepath: &Path) -> (Vec<Vec<char>>, (usize, usize)) {
    let manifold: Vec<Vec<char>> = std::fs::read_to_string(filepath).unwrap().lines().map(|line| line.chars().collect()).collect();
    let beam_position = (0, manifold.first().unwrap().iter().position(|x| *x == 'S').unwrap());

    (manifold, beam_position)
}

fn draw_beams_and_calculate_splits(manifold: &mut Vec<Vec<char>>, beam_position: (usize, usize)) -> usize {
    let mut splits = 0;
    for row in beam_position.0..manifold.len() - 1 {
        for col in 0..manifold[row].len() {
            if manifold[row][col] != 'S' && manifold[row][col] != '|' { continue; }
            let next_beam_position = (row as i64 + 1, col as i64);
            let next_row_range = 0..manifold[next_beam_position.0 as usize].len() as i64;

            if !next_row_range.contains(&next_beam_position.1) { continue; }

            let next_beam_manifold = manifold[next_beam_position.0 as usize][next_beam_position.1 as usize];

            if next_beam_manifold == '^' || next_beam_manifold == '*' {
                manifold[next_beam_position.0 as usize][next_beam_position.1 as usize] = '*';

                if next_row_range.contains(&(next_beam_position.1 - 1)) { manifold[next_beam_position.0 as usize][next_beam_position.1 as usize - 1] = '|' }
                if next_row_range.contains(&(next_beam_position.1 + 1)) { manifold[next_beam_position.0 as usize][next_beam_position.1 as usize + 1] = '|' }
                splits += 1;
            } else {
                manifold[next_beam_position.0 as usize][next_beam_position.1 as usize] = '|'
            }
        }
    }

    splits
}

fn main() {
    let filepath = Path::new("day07/input.txt");
    let (mut manifold, beam_position) = read_tachyon_manifold_and_beam_position(filepath);
    println!("Tachyon manifold: {:?}", manifold);
    let splits = draw_beams_and_calculate_splits(&mut manifold, beam_position);
    println!("Number of beam splits: {}", splits)
}