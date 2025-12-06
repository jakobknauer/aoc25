use crate::common::read_input;

pub fn run() -> (u64, u64) {
    let input = read_input(6);
    let lines: Vec<&str> = input.lines().collect();

    let part1 = part1(&lines);
    let part2 = part2(&lines);

    (part1, part2)
}

fn part1(lines: &[&str]) -> u64 {
    let numbers: Vec<Vec<u64>> = lines
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|line| line.split(' ').filter_map(|n| n.parse().ok()).collect())
        .collect();
    let operators: Vec<char> = lines
        .last()
        .unwrap()
        .split(' ')
        .filter_map(|op| op.chars().next())
        .collect();

    let n_equations = numbers[0].len();

    (0..n_equations)
        .map(|index| -> u64 {
            let operands = numbers.iter().map(|n| n[index]);
            if operators[index] == '+' {
                operands.sum()
            } else if operators[index] == '*' {
                operands.product()
            } else {
                panic!("Unexpected operator: {}", operators[index])
            }
        })
        .sum()
}

fn part2(lines: &[&str]) -> u64 {
    let numbers: Vec<Vec<char>> = lines
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|line| line.chars().collect())
        .collect();

    let op_line = *lines.last().unwrap();

    let operator_columns: Vec<_> = op_line
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| ['+', '*'].contains(&c).then_some(idx))
        .collect();

    let mut total = 0;
    for (i, &op_col) in operator_columns.iter().enumerate() {
        let last_operand_col = operator_columns
            .get(i + 1)
            .map(|col| col - 1)
            .unwrap_or(numbers[0].len());

        let mut operands = Vec::<u64>::new();
        for col in op_col..last_operand_col {
            let operand = numbers
                .iter()
                .map(|line| line[col])
                .filter_map(|c| c.is_ascii_digit().then_some((c as i64) - ('0' as i64)))
                .rev()
                .enumerate()
                .map(|(i, digit)| 10_u64.pow(i as u32) * (digit as u64))
                .sum();
            operands.push(operand);
        }

        let operator = op_line.chars().nth(op_col).unwrap();
        if operator == '+' {
            total += operands.iter().sum::<u64>()
        } else if operator == '*' {
            total += operands.iter().product::<u64>()
        } else {
            panic!("Unexpected operator: {}", operator);
        }
    }

    total
}

mod test {
    #[test]
    fn part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let result = super::part2(&input.lines().collect::<Vec<_>>());
        assert_eq!(result, 3263827);
    }
}
