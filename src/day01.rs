use crate::common::read_input;

enum Direction {
    Left,
    Right,
}

struct Instruction {
    dir: Direction,
    distance: i32,
}

pub fn run() {
    let (_, part1, part2) = read_input(1)
        .lines()
        .map(|line| {
            let (dir, rest) = line.split_at(1);
            let dir = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let distance = rest.parse().unwrap();
            Instruction { dir, distance }
        })
        .fold(
            (50, 0, 0),
            |(mut current, mut part1, mut part2), Instruction { dir, distance }| {
                let dir = match dir {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };

                for _ in 0..distance {
                    current += dir;
                    current %= 100;
                    if current == 0 {
                        part2 += 1
                    }
                }

                if current == 0 {
                    part1 += 1;
                }

                (current, part1, part2)
            },
        );

    println!("{part1} {part2}");
}
