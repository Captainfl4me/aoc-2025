use std::collections::{HashMap, VecDeque};

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

fn count_path_to_out<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, start_node: &'a str, mut has_visit_fft: bool, mut has_visit_dac: bool, cache: &mut HashMap<(&'a str, bool, bool), u64>) -> u64 {
    if start_node == "fft" {
        has_visit_fft = true;
    } else if start_node == "dac" {
        has_visit_dac = true;
    }

    if start_node == "out" {
        if has_visit_fft && has_visit_dac { 1 } else { 0 }
    } else if let Some(cached_val) = cache.get(&(start_node, has_visit_fft, has_visit_dac)) {
        *cached_val
    } else {

        let res = graph.get(start_node).unwrap().iter().fold(0, |acc, &child| {
            acc + count_path_to_out(graph, child, has_visit_fft, has_visit_dac, cache)
        });

        cache.insert((start_node, has_visit_fft, has_visit_dac), res);

        res
    }
}

fn part_2(input: &str) -> u64 {
    let mut nodes_list: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut cache: HashMap<(&str, bool, bool), u64> = HashMap::new();

    for line in input.lines() {
        let mut next = Vec::new();

        for connections in line[5..].split(' ') {
            next.push(connections);
        }

        nodes_list.insert(line[0..3].as_ref(), next);
    }


    count_path_to_out(&nodes_list, "svr", false, false, &mut cache)
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
