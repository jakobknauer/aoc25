use crate::common::read_input;

pub fn run() {
    let ranges: Vec<(usize, usize)> = read_input(2)
        .contents()
        .trim()
        .split(',')
        .map(|range| {
            let (a, b) = range.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let part1: usize = ranges
        .iter()
        .flat_map(|&(a, b)| a..=b)
        .filter(|&id| !is_valid(id))
        .sum();

    let part2: usize = ranges
        .iter()
        .flat_map(|&(a, b)| a..=b)
        .filter(|&id| !is_valid_2(id))
        .sum();

    println!("{part1} {part2}");
}

fn is_valid(id: usize) -> bool {
    let length = id.to_string().len();
    length % 2 == 1 || !is_partitioned(id, length / 2, 2)
}

fn is_valid_2(id: usize) -> bool {
    let length = id.to_string().len();

    [2, 3, 5, 7]
        .into_iter()
        .filter(|&n_parts| length.is_multiple_of(n_parts))
        .all(|n_parts| !is_partitioned(id, length / n_parts, n_parts))
}

fn is_partitioned(id: usize, part_length: usize, n_parts: usize) -> bool {
    let first_part = decimal_tail(id, part_length);

    let target = (0..n_parts)
        .map(|i| first_part * 10_usize.pow((i * part_length) as u32))
        .sum();

    id == target
}

fn decimal_tail(n: usize, n_digits: usize) -> usize {
    n % 10_usize.pow(n_digits as u32)
}
