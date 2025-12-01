enum Direction {
    Left,
    Right,
}
struct Instruction {
    dir: Direction,
    distance: i32,
}

pub fn day01() {
    let input_path = "input/day01.txt";

    let instructions: Vec<_> = std::fs::read_to_string(input_path)
        .unwrap()
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
        .collect();

    let mut current = 50;
    let mut part1 = 0;
    let mut part2 = 0;
    for Instruction { dir, distance } in instructions {
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
    }

    println!("{part1} {part2}");
}
