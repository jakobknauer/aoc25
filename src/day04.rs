use itertools::Itertools;

use crate::common::grid::Grid;
use crate::common::read_input;

pub fn run() {
    let mut grid: Grid<bool> = read_input(4).lines().map(|line| line.chars().map(|c| c == '@')).into();

    let part1 = get_removable_paperrolls(&grid).into_iter().count();

    let mut part2 = 0;
    loop {
        let removable: Vec<_> = get_removable_paperrolls(&grid).into_iter().collect();
        if removable.is_empty() {
            break;
        }

        part2 += removable.len();
        for (r, c) in removable {
            grid.set(r, c, false);
        }
    }

    println!("{part1} {part2}");
}

fn get_removable_paperrolls(grid: &Grid<bool>) -> impl IntoIterator<Item = (usize, usize)> {
    (0..grid.height()).cartesian_product(0..grid.width()).filter(|&(r, c)| {
        *grid.get(r, c)
            && grid
                .get_neighbors(r as isize, c as isize)
                .into_iter()
                .filter(|&&n| n)
                .count()
                < 4
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1() {
        let grid: Grid<bool> = SAMPLE.lines().map(|line| line.chars().map(|c| c == '@')).into();
        let part1 = get_removable_paperrolls(&grid).into_iter().count();
        assert_eq!(part1, 13);
    }
}
