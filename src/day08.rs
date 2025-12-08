use std::collections::HashMap;

use itertools::Itertools;

use crate::common::read_input;

pub fn run() -> (usize, i64) {
    let boxes: Vec<[i64; 3]> = read_input(8)
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let num_boxes = boxes.len();
    let mut pairs: Vec<_> = (0..num_boxes)
        .cartesian_product(0..num_boxes)
        .filter(|(a, b)| a < b)
        .collect();
    pairs.sort_by_key(|&(a, b)| boxes[a].iter().zip(boxes[b]).map(|(x, y)| (x - y).pow(2)).sum::<i64>());

    let part1 = part1(&pairs, num_boxes, 1000);
    let part2 = part2(&pairs, &boxes);

    (part1, part2)
}

fn part1(pairs: &[(usize, usize)], num_boxes: usize, n_connections: usize) -> usize {
    // Size of all currently existing clusters
    let mut circuit_sizes: Vec<usize> = vec![1; num_boxes];
    // Point to which cluster a certain box belongs
    let mut circuit_pointer: HashMap<usize, usize> = (0..num_boxes).map(|idx| (idx, idx)).collect();

    for &(a, b) in pairs.iter().take(n_connections) {
        let mut head_a = a;
        let mut head_b = b;

        // Find canonical representative for 'a'
        while circuit_pointer[&head_a] != head_a {
            head_a = circuit_pointer[&head_a];
        }
        // Find canonical representative for 'b'
        while circuit_pointer[&head_b] != head_b {
            head_b = circuit_pointer[&head_b];
        }
        //
        // Some micro-optimization
        *circuit_pointer.get_mut(&b).unwrap() = head_a;
        *circuit_pointer.get_mut(&a).unwrap() = head_a;

        if head_a == head_b {
            continue;
        }

        // Unify clusters a and b
        *circuit_pointer.get_mut(&head_b).unwrap() = head_a;
        let cluster_b = std::mem::take(circuit_sizes.get_mut(head_b).unwrap());
        *circuit_sizes.get_mut(head_a).unwrap() += cluster_b;
    }

    circuit_sizes.sort();
    circuit_sizes.iter().rev().take(3).product()
}

fn part2(pairs: &[(usize, usize)], boxes: &[[i64; 3]]) -> i64 {
    let num_boxes = boxes.len();

    // Size of all currently existing clusters
    let mut circuit_sizes: Vec<usize> = vec![1; num_boxes];
    // Point to which cluster a certain box belongs
    let mut circuit_pointer: HashMap<usize, usize> = (0..num_boxes).map(|idx| (idx, idx)).collect();

    let mut empty_circuits = 0;

    for &(a, b) in pairs {
        let mut head_a = a;
        let mut head_b = b;
        // Find canonical representative for 'a'
        while circuit_pointer[&head_a] != head_a {
            head_a = circuit_pointer[&head_a];
        }
        // Find canonical representative for 'b'
        while circuit_pointer[&head_b] != head_b {
            head_b = circuit_pointer[&head_b];
        }

        // Some micro-optimization
        *circuit_pointer.get_mut(&b).unwrap() = head_a;
        *circuit_pointer.get_mut(&a).unwrap() = head_a;

        if head_a == head_b {
            continue;
        }

        // Unify clusters a and b
        *circuit_pointer.get_mut(&head_b).unwrap() = head_a;
        let cluster_b = std::mem::take(circuit_sizes.get_mut(head_b).unwrap());
        *circuit_sizes.get_mut(head_a).unwrap() += cluster_b;

        // Check break condition
        empty_circuits += 1;
        if empty_circuits + 1 == num_boxes {
            return boxes[a][0] * boxes[b][0];
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let boxes = [
            [162, 817, 812],
            [57, 618, 57],
            [906, 360, 560],
            [592, 479, 940],
            [352, 342, 300],
            [466, 668, 158],
            [542, 29, 236],
            [431, 825, 988],
            [739, 650, 466],
            [52, 470, 668],
            [216, 146, 977],
            [819, 987, 18],
            [117, 168, 530],
            [805, 96, 715],
            [346, 949, 466],
            [970, 615, 88],
            [941, 993, 340],
            [862, 61, 35],
            [984, 92, 344],
            [425, 690, 689],
        ];

        let actual = part1(&boxes, 10);
        assert_eq!(actual, 40);
    }
}
