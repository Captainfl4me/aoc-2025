use itertools::Itertools;

fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-09/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let area = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap()
        })
        .combinations(2)
        .map(|comb| {
            let width = (comb[0].0 - comb[1].0).abs() + 1;
            let height = (comb[0].1 - comb[1].1).abs() + 1;
            width * height
        });

    area.max().unwrap() as u64
}

fn part_2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests_day_09 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-09/test.txt");
        assert_eq!(part_1(input), 50);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-09/test.txt");
        assert_eq!(part_2(input), 0);
    }
}
