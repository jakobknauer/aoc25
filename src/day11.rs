use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::read_input;

pub fn run() -> (u64, u64) {
    let devices: HashMap<String, Vec<String>> = read_input(11)
        .lines()
        .map(|line| {
            let (device, connections) = line.split_once(':').unwrap();
            let device = device.to_string();
            let connections = connections.trim().split(' ').map(String::from).collect();
            (device, connections)
        })
        .collect();

    let sorting = compute_topological_sorting(&devices);
    let part1 = count_paths(&devices, &sorting, "you", "out");
    let part2 = count_routes(&devices, &sorting, &["svr", "dac", "fft", "out"])
        + count_routes(&devices, &sorting, &["svr", "fft", "dac", "out"]);

    (part1, part2)
}

fn compute_topological_sorting(devices: &HashMap<String, Vec<String>>) -> Vec<&str> {
    let mut sorted: Vec<&str> = Vec::new();
    let mut temporary_marks: HashSet<&str> = HashSet::new();
    let mut permanent_marks: HashSet<&str> = HashSet::new();

    while let Some(unmarked) = devices.keys().find(|device| !permanent_marks.contains(device.as_str())) {
        visit(
            devices,
            unmarked,
            &mut permanent_marks,
            &mut temporary_marks,
            &mut sorted,
        );
    }

    sorted.reverse();
    sorted
}

fn visit<'a>(
    devices: &'a HashMap<String, Vec<String>>,
    device: &'a str,
    permanent_marks: &mut HashSet<&'a str>,
    temporary_marks: &mut HashSet<&'a str>,
    sorted: &mut Vec<&'a str>,
) {
    if permanent_marks.contains(device) {
        return;
    }
    if temporary_marks.contains(device) {
        panic!("Graph has a cycle");
    }

    temporary_marks.insert(device);

    for next_device in devices.get(device).iter().cloned().flatten() {
        visit(devices, next_device, permanent_marks, temporary_marks, sorted);
    }

    permanent_marks.insert(device);
    sorted.push(device);
}

fn count_paths(devices: &HashMap<String, Vec<String>>, sorting: &[&str], from: &str, to: &str) -> u64 {
    let mut path_count: HashMap<&str, u64> = HashMap::new();
    path_count.insert(from, 1);

    for &current in sorting {
        if current == to {
            break;
        }
        for next in devices.get(current).iter().cloned().flatten() {
            *path_count.entry(next).or_default() += path_count.get(current).cloned().unwrap_or(0);
        }
    }

    path_count.get(to).cloned().unwrap_or(0)
}

fn count_routes(devices: &HashMap<String, Vec<String>>, sorting: &[&str], route: &[&str]) -> u64 {
    route
        .iter()
        .tuple_windows()
        .map(|(a, b)| count_paths(devices, sorting, a, b))
        .product()
}
