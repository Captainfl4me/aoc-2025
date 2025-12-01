fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-01/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    // dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let mut dial : i32 = 50;
    let mut code : u64 = 0;

    for line in input.lines() {
        let val = line[1..].parse::<i32>().unwrap();

        if line.starts_with('L') {
            dial -= val;
        }else if line.starts_with('R') {
            dial += val;
        }

        while dial < 0 {
            dial += 100;
        }

        while dial > 99 {
            dial -= 100;
        }

        if dial == 0 {
            code += 1;
        }
    }

    code
}

fn part_2(input: &str) -> u64 {
    todo!()
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
        assert_eq!(part_2(input), 0);
    }
}
