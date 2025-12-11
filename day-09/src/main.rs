fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-09/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    todo!()
}

fn part_2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests_day_09 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-09/test.txt");
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-09/test.txt");
        assert_eq!(part_2(input), 0);
    }
}
