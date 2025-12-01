fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-01/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let mut dial: i32 = 50;
    let mut code: u64 = 0;

    for line in input.lines() {
        let val = line[1..].parse::<i32>().unwrap();

        if line.starts_with('L') {
            dial -= val;
        } else if line.starts_with('R') {
            dial += val;
        }

        dial %= 100;

        if dial == 0 {
            code += 1;
        }
    }

    code
}

fn part_2(input: &str) -> u64 {
    let mut dial: i64 = 50;
    let mut code: i64 = 0;

    for line in input.lines() {
        let val = line[1..].parse::<i64>().unwrap();
        let mut mult = 1;

        if line.starts_with('L') {
            mult = -1;
        }

        for _ in 0..val {
            dial = (dial + mult) % 100;

            if dial == 0 {
                code += 1;
            }
        }
    }

    u64::try_from(code).unwrap()
}

#[cfg(test)]
mod tests_day_01 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-01/test.txt");
        assert_eq!(part_1(input), 3);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-01/test.txt");
        assert_eq!(part_2(input), 6);
    }

    #[test]
    fn test_part_2_edge() {
        let input = "L50\nR50\nL50\nR50";
        assert_eq!(part_2(input), 2);

        let input = "L50\nL100";
        assert_eq!(part_2(input), 2);

        let input = "R50\nL100";
        assert_eq!(part_2(input), 2);

        let input = "L50\nL100";
        assert_eq!(part_2(input), 2);

        let input = "L50\nL200";
        assert_eq!(part_2(input), 3);

        let input = "L200";
        assert_eq!(part_2(input), 2);

        let input = "R1000";
        assert_eq!(part_2(input), 10);
    }
}
