use std::collections::{HashSet, LinkedList};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-08/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input, 1000));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

#[derive(Debug)]
struct link {
    point1: (i64, i64, i64),
    point2: (i64, i64, i64),
    dist: f64,
}

fn compute_eucl_dist(point1: &(i64, i64, i64), point2: &(i64, i64, i64)) -> f64 {
    (((point1.0 - point2.0).pow(2) + (point1.1 - point2.1).pow(2) + (point1.2 - point2.2).pow(2))
        as f64)
        .sqrt()
}

fn part_1(input: &str, iter_cnt: u64) -> u64 {
    let mut dist = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64, i64)>()
                .unwrap()
        })
        .combinations(2)
        .map(|combination| link {
            point1: combination[0],
            point2: combination[1],
            dist: compute_eucl_dist(&combination[0], &combination[1]),
        })
        .sorted_by_key(|link| link.dist as u64);

    let mut circuits: Vec<HashSet<(i64, i64, i64)>> = Vec::new();

    for _ in 0..iter_cnt {
        if let Some(link) = dist.next() {
            let mut find_circuit: Option<usize> = None;
            let mut circuit_to_merge: Option<usize> = None;

            for (index, circuit) in circuits.iter_mut().enumerate() {
                if circuit.contains(&link.point1) || circuit.contains(&link.point2) {
                    if find_circuit.is_some() {
                        circuit_to_merge = Some(index);
                    } else {
                        circuit.insert(link.point1);
                        circuit.insert(link.point2);
                        find_circuit = Some(index);
                    }
                }
            }

            if let Some(ctm) = circuit_to_merge {
                let old_circuit = circuits[ctm].clone();
                circuits[find_circuit.unwrap()].extend(old_circuit);
                circuits.swap_remove(ctm);
            }

            if find_circuit.is_none() {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(link.point1);
                new_circuit.insert(link.point2);
                circuits.push(new_circuit);
            }
        }
    }

    circuits.sort_by_cached_key(|circuit| circuit.len());
    circuits.reverse();

    circuits[0..3]
        .iter()
        .fold(1, |acc, circuit| acc * circuit.len()) as u64
}

fn part_2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests_day_08 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-08/test.txt");
        assert_eq!(part_1(input, 10), 40);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-08/test.txt");
        assert_eq!(part_2(input), 0);
    }
}
