use day07::*;
use std::path::Path;

fn count_possible_paths(node: &Node) -> usize {
    if node.children.is_empty() {
        return 1;
    }

    let possible_paths = node
        .children
        .iter()
        .map(|child| count_possible_paths(&*child.borrow()))
        .sum();
    possible_paths
}

// fn count_possible_paths(manifold: &Vec<Vec<ManifoldField>>, row: usize, col: usize) -> usize {
//     let mut row = row;

//     while row < manifold.len() {
//         match manifold[row][col] {
//             ManifoldField::Splitter => {
//                 let left = count_possible_paths(manifold, row, col - 1);
//                 let right = count_possible_paths(manifold, row, col + 1);

//                 return left + right;
//             }
//             _ => row += 1,
//         }
//     }

//     1
// }

fn main() {
    let filepath = Path::new("day07/example.txt");
    let (mut manifold, root) = read_tachyon_manifold_with_beam_root(filepath);
    println!("Tachyon manifold: {:?}", manifold);
    let _ = draw_beams_and_calculate_splits(&mut manifold);
    println!("Changed tachyon manifold: {:?}", manifold);
    let possible_paths = count_possible_paths(&*root.borrow());
    // let beam_position = manifold
    //     .first()
    //     .unwrap()
    //     .iter()
    //     .position(|x| x == ManifoldField::Beam)
    //     .unwrap();
    // let possible_paths = count_possible_paths(&manifold, 0, beam_position);
    println!("Number of possible paths: {}", possible_paths)
}
