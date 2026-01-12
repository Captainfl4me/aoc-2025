use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-10/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| {
        let str_group: Vec<String> = line.split(' ').map(|str| str.to_string()).collect::<Vec<String>>();
        let mask: u32 = str_group[0][1..].chars().enumerate().fold(0, |mask, (i, char)| {
            mask | if char == '#' { 1 << i } else { 0 }
        });

        let mut buttons: HashSet<u32> = HashSet::new();
        for str_list in str_group.iter().skip(1).take(str_group.len()-2) {
            let btn_mask: u32 = str_list[1..(str_list.len()-1)].split(',').map(|str| str.parse::<u8>().unwrap()).fold(0, |mask, i| {
                mask | (1 << i)
            });
            buttons.insert(btn_mask);
        }

        for k in 1..buttons.len() {
            let matching_comb = buttons.iter().combinations(k).map(|comb| comb.iter().fold(0, |curr, &&btn| curr ^ btn)).filter(|&x| x == mask).count();
            if matching_comb > 0 {
                return acc + (k as u64);
            }
        }

        acc
    })
}

fn part_2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests_day_10 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-10/test.txt");
        assert_eq!(part_1(input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-10/test.txt");
        assert_eq!(part_2(input), 0);
    }
}
