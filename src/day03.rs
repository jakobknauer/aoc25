use crate::common::read_input;

pub fn run() -> (u64, u64) {
    let banks: Vec<Vec<u64>> = read_input(3)
        .lines()
        .map(|bank| bank.chars().map(|c| c as u64 - '0' as u64).collect())
        .collect();

    let part1 = banks.iter().map(|bank| joltage(bank, 2)).sum();
    let part2 = banks.iter().map(|bank| joltage(bank, 12)).sum();

    (part1, part2)
}

fn joltage(mut bank: &[u64], n: usize) -> u64 {
    let mut total = 0;
    for i in (1..=n).rev() {
        let digit_idx = argmax_capped_at_9(&bank[..=(bank.len() - i)]);
        total = total * 10 + bank[digit_idx];
        bank = &bank[digit_idx + 1..];
    }
    total
}

fn argmax_capped_at_9(bank: &[u64]) -> usize {
    let mut max_idx = 0;
    for (idx, &val) in bank.iter().enumerate() {
        if val > bank[max_idx] {
            max_idx = idx;
        }
        if val == 9 {
            break;
        }
    }
    max_idx
}
