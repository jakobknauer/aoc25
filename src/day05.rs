use crate::common::read_input;

pub fn run() -> (usize, u64) {
    let input = read_input(5);

    let mut lines = input.lines();

    let mut intervals: Vec<(u64, u64)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let ingredients: Vec<u64> = lines.map(|line| line.parse().unwrap()).collect();

    let part1 = part1(&intervals, &ingredients);
    let part2 = part2(&mut intervals);

    (part1, part2)
}

fn part1(intervals: &[(u64, u64)], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|&&ing| intervals.iter().any(|&(start, end)| (start <= ing) && (ing <= end)))
        .count()
}

fn part2(intervals: &mut [(u64, u64)]) -> u64 {
    intervals.sort();
    let mut it = intervals.iter().copied().peekable();

    let mut total = 0;

    while let Some((start, mut end)) = it.next() {
        while let Some((s, e)) = it.peek().cloned()
            && s <= end
        {
            end = end.max(e);
            it.next();
        }

        total += end - start + 1;
    }

    total
}
