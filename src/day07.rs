use std::collections::HashMap;

use crate::common::{grid::Grid, read_input};

pub fn run() -> (usize, usize) {
    let manifold: Grid<char> = read_input(7).lines().map(|line| line.chars()).into();

    let start = (0..manifold.width()).find(|&col| manifold[(0, col)] == 'S').unwrap();
    let mut timelines: HashMap<usize, usize> = [(start, 1)].into();

    let mut num_splits = 0;

    for row in 1..manifold.height() {
        let mut new_timelines = HashMap::new();
        for (beam, count) in timelines {
            if manifold[(row, beam)] == '.' {
                *new_timelines.entry(beam).or_default() += count;
            } else if manifold[(row, beam)] == '^' {
                num_splits += 1;
                if beam > 0 {
                    *new_timelines.entry(beam - 1).or_default() += count;
                }
                if beam + 1 < manifold.width() {
                    *new_timelines.entry(beam + 1).or_default() += count;
                }
            }
        }
        timelines = new_timelines;
    }

    let part1 = num_splits;
    let part2 = timelines.values().sum();
    (part1, part2)
}
