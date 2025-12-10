use std::collections::{HashSet, VecDeque};

use z3::ast::Int;

use crate::common::read_input;

pub fn run() -> (u32, u32) {
    let machines: Vec<Machine> = read_input(10).lines().map(parse_machine).collect();

    let part1 = machines.iter().map(part1).sum();
    let part2 = machines.iter().map(part2).sum();

    (part1, part2)
}

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<bool>>,
    joltages: Vec<u32>,
}

fn parse_machine(line: &str) -> Machine {
    let parts: Vec<_> = line.split(' ').collect();

    let lights: Vec<bool> = {
        let lights = parts[0];
        lights[1..lights.len() - 1]
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!(),
            })
            .collect()
    };

    let buttons = parts[1..parts.len() - 1]
        .iter()
        .map(|button| {
            let button = &button[1..button.len() - 1];
            let button_lights: HashSet<usize> = button.split(',').map(|l| l.parse().unwrap()).collect();
            (0..lights.len()).map(|index| button_lights.contains(&index)).collect()
        })
        .collect();

    let joltages = {
        let joltages = parts[parts.len() - 1];
        joltages[1..joltages.len() - 1]
            .split(',')
            .map(|j| j.parse().unwrap())
            .collect()
    };

    Machine {
        lights,
        buttons,
        joltages,
    }
}

fn part1(machine: &Machine) -> u32 {
    // Dijkstra/Floodfill implementation
    let mut open: VecDeque<(Vec<bool>, u32)> = VecDeque::new();
    let mut closed: HashSet<Vec<bool>> = HashSet::new();

    open.push_back((vec![false; machine.lights.len()], 0));

    while let Some((state, distance)) = open.pop_front() {
        assert_eq!(state.len(), machine.lights.len());

        if state == machine.lights {
            return distance;
        }

        if closed.contains(&state) {
            continue;
        }
        closed.insert(state.clone());

        for button in &machine.buttons {
            assert_eq!(state.len(), button.len());

            let xor = state.iter().zip(button).map(|(s, b)| s ^ b).collect();
            open.push_back((xor, distance + 1));
        }
    }

    unreachable!()
}

fn part2(machine: &Machine) -> u32 {
    let opt = z3::Optimize::new();

    let presses: Vec<Int> = (0..machine.buttons.len())
        .map(|j| Int::new_const(format!("press_{}", j)))
        .collect();
    for press in &presses {
        opt.assert(&press.ge(Int::from_u64(0)));
    }
    let press_sum = Int::add(&presses);

    for (idx, &joltage) in machine.joltages.iter().enumerate() {
        let mut exprs = vec![];
        for (i, button) in machine.buttons.iter().enumerate() {
            if button[idx] {
                exprs.push(presses[i].clone());
            }
        }
        let sum = Int::add(&exprs);
        opt.assert(&sum.eq(Int::from_u64(joltage as u64)));
    }

    opt.minimize(&press_sum);
    match opt.check(&[]) {
        z3::SatResult::Sat => {
            let model = opt.get_model().unwrap();
            model.eval(&press_sum, true).unwrap().as_i64().unwrap() as u32
        }
        _ => panic!("No solution found"),
    }
}
