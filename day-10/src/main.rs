use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-10/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| {
        let str_group: Vec<String> = line
            .split(' ')
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
        let mask: u32 = str_group[0][1..]
            .chars()
            .enumerate()
            .fold(0, |mask, (i, char)| {
                mask | if char == '#' { 1 << i } else { 0 }
            });

        let mut buttons: HashSet<u32> = HashSet::new();
        for str_list in str_group.iter().skip(1).take(str_group.len() - 2) {
            let btn_mask: u32 = str_list[1..(str_list.len() - 1)]
                .split(',')
                .map(|str| str.parse::<u8>().unwrap())
                .fold(0, |mask, i| mask | (1 << i));
            buttons.insert(btn_mask);
        }

        for k in 1..buttons.len() {
            let matching_comb = buttons
                .iter()
                .combinations(k)
                .map(|comb| comb.iter().fold(0, |curr, &&btn| curr ^ btn))
                .filter(|&x| x == mask)
                .count();
            if matching_comb > 0 {
                return acc + (k as u64);
            }
        }

        acc
    })
}

fn find_min_comb_joltage(buttons: &HashSet<u32>, joltages: &[u32]) -> u64 {
    if joltages.iter().all(|x| *x == 0) {
        0
    } else {
        let odd_mask = joltages.iter().enumerate().fold(0, |mask, (i, &n)| {
            mask | if (n % 2) == 1 { 1 << i } else { 0 }
        });

        let mut next_states: Vec<(Vec<u32>, u64)> = Vec::new();

        if odd_mask == 0 {
            next_states.push((joltages.iter().map(|x| x / 2).collect_vec(), 0));
        }

        for k in 1..=buttons.len() {
            let matching_comb = buttons.iter().combinations(k);

            for comb in matching_comb {
                if comb.iter().fold(0, |curr, &&btn| curr ^ btn) == odd_mask {
                    let comb_count = comb.len() as u64;
                    let mut new_joltages: Vec<u32> = joltages.to_owned();
                    let mut comb_is_illegal = false;

                    for &&btn in comb.iter() {
                        for (i, joltage) in new_joltages.iter_mut().enumerate() {
                            if (btn & (1 << i)) != 0 {
                                if *joltage == 0 {
                                    comb_is_illegal = true;
                                } else {
                                    *joltage -= 1;
                                }
                            }
                        }
                    }

                    if !comb_is_illegal {
                        next_states
                            .push((new_joltages.iter().map(|x| x / 2).collect_vec(), comb_count));
                    }
                }
            }
        }

        next_states
            .iter()
            .map(|(next_joltages, comb_count)| {
                2 * find_min_comb_joltage(buttons, next_joltages) + *comb_count
            })
            .min()
            .unwrap_or(1000000)
    }
}

fn part_2(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| {
        let str_group: Vec<String> = line
            .split(' ')
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
        let str_joltages = str_group.last().unwrap();
        let counters: Vec<u32> = str_joltages[1..(str_joltages.len() - 1)]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_vec();

        let mut buttons: HashSet<u32> = HashSet::new();
        for str_list in str_group.iter().skip(1).take(str_group.len() - 2) {
            let btn_mask: u32 = str_list[1..(str_list.len() - 1)]
                .split(',')
                .map(|str| str.parse::<u8>().unwrap())
                .fold(0, |mask, i| mask | (1 << i));
            buttons.insert(btn_mask);
        }

        let val = find_min_comb_joltage(&buttons, &counters);
        acc + val
    })
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
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part_2(input), 11);
        // let input = include_str!("../../aoc-2025-inputs/day-10/test.txt");
        // assert_eq!(part_2(input), 33);
    }
}
