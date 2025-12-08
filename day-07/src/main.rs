use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-07/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let mut lines_iter = input.lines().map(|line| line.chars().collect::<Vec<char>>());
    let first_beam = lines_iter.next().unwrap().iter().position(|&x| x == 'S').unwrap();

    let mut split = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(first_beam);
    for line in lines_iter {
        let mut new_beams: HashSet<usize> = HashSet::new();
        for &beam in beams.iter() {
            if line[beam] == '^' {
                new_beams.insert(beam-1);
                new_beams.insert(beam+1);
                split += 1;
            } else {
                new_beams.insert(beam);
            }
        }

        beams = new_beams;
    }    

    split
}

fn part_2(input: &str) -> u64 {
    let mut lines_iter = input.lines().map(|line| line.chars().collect::<Vec<char>>());
    let first_beam = lines_iter.next().unwrap().iter().position(|&x| x == 'S').unwrap();

    let mut beams: HashMap<usize, u64> = HashMap::new();
    beams.insert(first_beam, 1);
    for line in lines_iter {
        let mut new_beams: HashMap<usize, u64> = HashMap::new();
        for (&beam, &n_path) in beams.iter() {
            if line[beam] == '^' {
                new_beams.entry(beam-1).and_modify(|n| *n += n_path).or_insert(n_path);
                new_beams.entry(beam+1).and_modify(|n| *n += n_path).or_insert(n_path);
            } else {
                new_beams.entry(beam).and_modify(|n| *n += n_path).or_insert(n_path);
            }
        }

        beams = new_beams;
    }    

    beams.values().sum()
}

#[cfg(test)]
mod tests_day_07 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-07/test.txt");
        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-07/test.txt");
        assert_eq!(part_2(input), 40);
    }
}
