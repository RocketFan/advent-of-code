use core::fmt;
use std::{cell::RefCell, ops::DerefMut, path::Path, rc::Rc, str::FromStr};

#[derive(Clone)]
pub struct Node {
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            children: Vec::new(),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node [{}]", self.children.len())
    }
}

#[derive(Debug, Clone)]
pub enum ManifoldField {
    Empty,
    Beam(Rc<RefCell<Node>>),
    Splitter,
}

impl FromStr for ManifoldField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(ManifoldField::Beam(Rc::new(RefCell::new(Node::default())))),
            "." => Ok(ManifoldField::Empty),
            "^" => Ok(ManifoldField::Splitter),
            _ => Err(format!("Unknown manifold field: {}", s)),
        }
    }
}

pub fn read_tachyon_manifold_with_beam_root(
    filepath: &Path,
) -> (Vec<Vec<ManifoldField>>, Rc<RefCell<Node>>) {
    let manifold: Vec<Vec<ManifoldField>> = std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let beam_root = manifold[0]
        .iter()
        .find_map(|x| {
            if let ManifoldField::Beam(node) = x {
                Some(node)
            } else {
                None
            }
        })
        .unwrap()
        .clone();
    (manifold, beam_root)
}

pub fn draw_beam_and_add_node(
    manifold: &mut Vec<Vec<ManifoldField>>,
    node: &mut Node,
    row: usize,
    col: usize,
) {
    let next_beam_field = &mut manifold[row][col];
    let next_node = match next_beam_field {
        ManifoldField::Beam(next_node) => next_node.clone(),
        ManifoldField::Empty => Rc::new(RefCell::new(Node::default())),
        _ => panic!("This shouldn't happen!"),
    };
    *next_beam_field = ManifoldField::Beam(next_node.clone());
    node.children.push(next_node);
}

pub fn draw_beams_and_calculate_splits(manifold: &mut Vec<Vec<ManifoldField>>) -> usize {
    let mut splits = 0;
    for row in 0..manifold.len() - 1 {
        for col in 0..manifold[row].len() {
            if let ManifoldField::Beam(node) = manifold[row][col].clone() {
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
                            draw_beam_and_add_node(
                                manifold,
                                &mut *node.borrow_mut(),
                                next_beam_position.0 as usize,
                                next_beam_position.1 as usize - 1,
                            );
                        }
                        if next_row_range.contains(&(next_beam_position.1 + 1)) {
                            draw_beam_and_add_node(
                                manifold,
                                &mut *node.borrow_mut(),
                                next_beam_position.0 as usize,
                                next_beam_position.1 as usize + 1,
                            );
                        }
                        splits += 1;
                    }
                    ManifoldField::Empty => {
                        draw_beam_and_add_node(
                            manifold,
                            &mut *node.borrow_mut(),
                            next_beam_position.0 as usize,
                            next_beam_position.1 as usize,
                        );
                    }
                    ManifoldField::Beam(next_node) => {
                        node.borrow_mut().deref_mut().children.push(next_node);
                    }
                }
            }
        }
    }

    splits
}
