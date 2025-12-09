use std::ops::Add;

use itertools::Itertools;

use crate::common::read_input;

pub fn run() -> (i64, i64) {
    let red_tiles: Vec<(usize, usize)> = read_input(9)
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let part1 = part1(&red_tiles);
    let part2 = part2(&red_tiles);

    (part1, part2)
}

fn part1(red_tiles: &[(usize, usize)]) -> i64 {
    let num_tiles = red_tiles.len();
    let pairs = (0..num_tiles)
        .flat_map(|a| (a + 1..num_tiles).map(move |b| (a, b)))
        .map(|(a, b)| (red_tiles[a], red_tiles[b]));

    pairs
        .map(|((x1, y1), (x2, y2))| (x1 as i64 - x2 as i64).abs().add(1) * (y1 as i64 - y2 as i64).abs().add(1))
        .max()
        .unwrap()
}

fn part2(red_tiles: &[(usize, usize)]) -> i64 {
    let num_tiles = red_tiles.len();
    let pairs = (0..num_tiles)
        .flat_map(|a| (a + 1..num_tiles).map(move |b| (a, b)))
        .map(|(a, b)| (red_tiles[a], red_tiles[b]));

    let min_x = red_tiles.iter().map(|&(x, _)| x).min().unwrap();

    let mut current_max = 0;
    for (c1, c2) in pairs {
        let (x1, y1) = c1;
        let (x2, y2) = c2;
        let area = (x1 as i64 - x2 as i64).abs().add(1) * (y1 as i64 - y2 as i64).abs().add(1);
        if area <= current_max {
            continue;
        }

        if is_rectangle_contained(c1, c2, red_tiles, min_x) {
            current_max = area;
        }
    }
    current_max
}

fn is_rectangle_contained(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    vertices: &[(usize, usize)],
    min_x: usize,
) -> bool {
    let diagonal_points = [(x1, y2), (x2, y1)];

    for &corner in &diagonal_points {
        if !is_point_contained(corner, vertices, min_x) {
            return false;
        }
    }

    let rect_edges = [
        ((x1, y1), (x1, y2)),
        ((x1, y2), (x2, y2)),
        ((x2, y2), (x2, y1)),
        ((x2, y1), (x1, y1)),
    ];

    for (&p1, &p2) in vertices.iter().tuple_windows() {
        for &(r1, r2) in &rect_edges {
            if edges_intersect_properly(p1, p2, r1, r2) {
                return false;
            }
        }
    }

    true
}

fn edges_intersect_properly(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    (x3, y3): (usize, usize),
    (x4, y4): (usize, usize),
) -> bool {
    if x1 == x2 && y3 == y4 {
        // edge1 vertical, edge2 horizontal
        x1 > x3.min(x4) && x1 < x3.max(x4) && y3 > y1.min(y2) && y3 < y1.max(y2)
    } else if y1 == y2 && x3 == x4 {
        // edge1 horizontal, edge2 vertical
        x3 > x1.min(x2) && x3 < x1.max(x2) && y1 > y3.min(y4) && y1 < y3.max(y4)
    } else {
        false
    }
}

fn edges_intersect_improperly(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    (x3, y3): (usize, usize),
    (x4, y4): (usize, usize),
) -> bool {
    if x1 == x2 && y3 == y4 {
        // edge1 vertical, edge2 horizontal
        x1 >= x3.min(x4) && x1 < x3.max(x4) && y3 >= y1.min(y2) && y3 < y1.max(y2)
    } else if y1 == y2 && x3 == x4 {
        // edge1 horizontal, edge2 vertical
        x3 >= x1.min(x2) && x3 < x1.max(x2) && y1 >= y3.min(y4) && y1 < y3.max(y4)
    } else {
        false
    }
}

fn is_point_contained(p: (usize, usize), vertices: &[(usize, usize)], min_x: usize) -> bool {
    let mut intersections = 0;

    for (&p1, &p2) in vertices.iter().tuple_windows() {
        if edges_intersect_improperly(p, (min_x - 1, p.1), p1, p2) {
            intersections += 1;
        }
    }

    intersections % 2 == 1
}
