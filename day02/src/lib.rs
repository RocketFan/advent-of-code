use itertools::Itertools;

pub fn read_ranges_from_file(filepath: &std::path::Path) -> Vec<(usize, usize)> {
    let input = std::fs::read_to_string(filepath).unwrap();
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for range in input.split(",") {
        if range.is_empty() {
            continue;
        }

        let range: (usize, usize) = range.split("-").map(|x| x.parse().unwrap()).collect_tuple().unwrap();
        ranges.push(range);
    }

    ranges
}
