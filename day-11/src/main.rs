use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-11/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let mut nodes_list: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut next = Vec::new();

        for connections in line[5..].split(' ') {
            next.push(connections);
        }

        nodes_list.insert(line[0..3].as_ref(), next);
    }

    let mut path_list: VecDeque<&str> = VecDeque::new();
    path_list.push_back("you");
    let mut path_cnt = 0;
    while !path_list.is_empty() {
        let node = path_list.pop_front().unwrap();

        if node == "out" {
            path_cnt += 1;
        } else if let Some(next) = nodes_list.get(node) {
            for &entry in next.iter() {
                path_list.push_back(entry);
            }
        }
    }

    path_cnt
}

struct State<'a> {
    visits: HashSet<&'a str>,
    current_node: &'a str,
}

fn part_2(input: &str) -> u64 {
    let mut nodes_list: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut next = Vec::new();

        for connections in line[5..].split(' ') {
            next.push(connections);
        }

        nodes_list.insert(line[0..3].as_ref(), next);
    }

    let mut path_list: VecDeque<State> = VecDeque::new();
    path_list.push_back(State {
        visits: {
            let mut hash = HashSet::new();
            hash.insert("svr");
            hash
        },
        current_node: "svr",
    });
    let mut path_cnt = 0;
    while !path_list.is_empty() {
        let mut node = path_list.pop_front().unwrap();

        if node.current_node == "out" {
            if node.visits.contains("fft") && node.visits.contains("dac") {
                path_cnt += 1;
            }
        } else if let Some(next) = nodes_list.get(node.current_node) {
            for &entry in next.iter() {
                if !node.visits.contains(entry) {
                    node.visits.insert(entry);
                    path_list.push_back(State {
                        visits: node.visits.clone(),
                        current_node: entry,
                    });
                }
            }
        }
    }

    path_cnt
}

#[cfg(test)]
mod tests_day_11 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-11/test.txt");
        assert_eq!(part_1(input), 5);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-11/test-2.txt");
        assert_eq!(part_2(input), 2);
    }
}
