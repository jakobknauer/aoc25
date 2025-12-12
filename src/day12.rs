use crate::common::read_input;

pub fn run() -> (u32, u32) {
    let input = read_input(12);
    let lines: Vec<_> = input.lines().collect();

    let mut shapes: Vec<Shape> = Vec::new();
    for block in 0..5 {
        let line1 = parse_block_line(lines[5 * block + 1]);
        let line2 = parse_block_line(lines[5 * block + 2]);
        let line3 = parse_block_line(lines[5 * block + 3]);
        shapes.push([line1, line2, line3]);
    }

    let mut part1 = 0;

    for region in &lines[30..] {
        let (size, counts) = region.split_once(':').unwrap();

        let (width, height) = size.split_once('x').unwrap();
        let width = width.parse().unwrap();
        let height = height.parse().unwrap();

        let counts: Vec<_> = counts.trim().split(' ').map(|c| c.parse().unwrap()).collect();

        if can_fit(width, height, counts, &shapes) {
            part1 += 1;
        }
    }

    (part1, 0)
}

type Shape = [[bool; 3]; 3];

fn parse_block_line(line: &str) -> [bool; 3] {
    let a = char_to_bool(line.chars().next().unwrap());
    let b = char_to_bool(line.chars().nth(1).unwrap());
    let c = char_to_bool(line.chars().nth(2).unwrap());
    [a, b, c]
}

fn char_to_bool(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!(),
    }
}

fn can_fit(width: usize, height: usize, counts: Vec<usize>, _shapes: &[Shape]) -> bool {
    let boxes = (width / 3) * (height / 3);
    let total_shapes = counts.iter().sum::<usize>();
    boxes >= total_shapes
}
