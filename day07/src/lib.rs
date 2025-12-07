use std::{path::Path, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum ManifoldField {
    Empty,
    Beam,
    Splitter,
}

impl FromStr for ManifoldField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(ManifoldField::Beam),
            "." => Ok(ManifoldField::Empty),
            "^" => Ok(ManifoldField::Splitter),
            _ => Err(format!("Unknown manifold field: {}", s)),
        }
    }
}

pub fn read_tachyon_manifold_with_beam_position(
    filepath: &Path,
) -> (Vec<Vec<ManifoldField>>, usize) {
    let manifold: Vec<Vec<ManifoldField>> = std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let beam_position = manifold[0]
        .iter()
        .position(|x| *x == ManifoldField::Beam)
        .unwrap()
        .clone();
    (manifold, beam_position)
}

pub fn draw_beams_and_calculate_splits(manifold: &mut Vec<Vec<ManifoldField>>) -> usize {
    let mut splits = 0;
    for row in 0..manifold.len() - 1 {
        for col in 0..manifold[row].len() {
            if ManifoldField::Beam != manifold[row][col] {
                continue;
            }
            let next_beam_position = (row as i64 + 1, col as i64);
            let next_row_range = 0..manifold[next_beam_position.0 as usize].len() as i64;

            if !next_row_range.contains(&next_beam_position.1) {
                continue;
            }

            let next_beam_field =
                manifold[next_beam_position.0 as usize][next_beam_position.1 as usize].clone();

            match next_beam_field {
                ManifoldField::Splitter => {
                    if next_row_range.contains(&(next_beam_position.1 - 1)) {
                        manifold[next_beam_position.0 as usize]
                            [next_beam_position.1 as usize - 1] = ManifoldField::Beam;
                    }
                    if next_row_range.contains(&(next_beam_position.1 + 1)) {
                        manifold[next_beam_position.0 as usize]
                            [next_beam_position.1 as usize + 1] = ManifoldField::Beam;
                    }
                    splits += 1;
                }
                _ => {
                    manifold[next_beam_position.0 as usize][next_beam_position.1 as usize] =
                        ManifoldField::Beam;
                }
            }
        }
    }

    splits
}
